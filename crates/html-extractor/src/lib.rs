use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    Expr, ExprStruct, Ident, LitStr, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Brace,
};

#[derive(Debug)]
struct HtmlCommands {
    commands: Vec<HtmlCommand>,
}

impl Parse for HtmlCommands {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut commands = Vec::new();
        while !input.is_empty() {
            commands.push(input.parse()?);
        }
        Ok(Self { commands })
    }
}

#[derive(Debug)]
enum HtmlCommand {
    ElementOpen(HtmlElement),
    Whitespace(HtmlWhitespace),
    ElementClose(HtmlElementClose),
    Comment(HtmlComment),
    Text(StringLiteralOrVariable),
    If(HtmlIf),
    While(HtmlWhile),
}

impl Parse for HtmlCommand {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![if]) {
            input.parse().map(Self::If)
        } else if lookahead.peek(Token![while]) {
            input.parse().map(Self::While)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Text)
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::Text)
        } else if lookahead.peek(Token![_]) {
            input.parse().map(Self::Whitespace)
        } else if lookahead.peek(Ident::peek_any) {
            input.parse().map(Self::Text)
        } else if lookahead.peek(Token![<]) {
            if input.peek2(Token![/]) {
                input.parse().map(Self::ElementClose)
            } else if input.peek2(Token![!]) {
                input.parse().map(Self::Comment)
            } else {
                input.parse().map(Self::ElementOpen)
            }
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
struct HtmlWhitespace {
    underscore: Token![_],
}

impl Parse for HtmlWhitespace {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { underscore: input.parse()? })
    }
}

#[derive(Debug)]
enum DashOrColon {
    Dash,
    Colon,
}

impl Parse for DashOrColon {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![-]) {
            input.parse().map(|_: Token![-]| Self::Dash)
        } else if lookahead.peek(Token![:]) {
            input.parse().map(|_: Token![:]| Self::Colon)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
enum StringLiteralOrVariable {
    Literal(LitStr),
    Variable(Ident),
    Expression(Expr),
}

impl Parse for StringLiteralOrVariable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let content;
            let _brace_token = braced!(content in input);

            content.parse().map(Self::Expression)
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Ident::peek_any) {
            input.call(Ident::parse_any).map(Self::Variable)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
struct HtmlAttribute {
    ident: Punctuated<Ident, DashOrColon>,
    value: StringLiteralOrVariable,
}

impl Parse for HtmlAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident: Punctuated<Ident, DashOrColon> = Punctuated::new();
        ident.push_value(input.call(Ident::parse_any)?);
        while input.peek(Token![-]) || input.peek(Token![:]) {
            ident.push_punct(input.parse()?);
            ident.push_value(input.parse()?);
        }
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { ident, value })
    }
}

#[derive(Debug)]
struct HtmlElement {
    element: Ident,
    attributes: Vec<HtmlAttribute>,
    open_end: Token![>],
}

impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        let element = input.call(Ident::parse_any)?;
        let mut attributes = Vec::new();
        while !input.peek(Token!(>)) {
            attributes.push(input.parse()?);
        }
        let open_end = input.parse()?;
        Ok(Self { element, attributes, open_end })
    }
}

#[derive(Debug)]
struct HtmlElementClose {
    element: Ident,
}

impl Parse for HtmlElementClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let element = input.call(Ident::parse_any)?;
        input.parse::<Token![>]>()?;
        Ok(Self { element })
    }
}

#[derive(Debug)]
struct HtmlComment {
    comment: LitStr,
}

impl Parse for HtmlComment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        input.parse::<Token![!]>()?;
        input.parse::<Token![-]>()?;
        input.parse::<Token![-]>()?;
        let comment: LitStr = input.parse()?;
        input.parse::<Token![-]>()?;
        input.parse::<Token![-]>()?;
        input.parse::<Token![>]>()?;
        Ok(Self { comment })
    }
}

// TODO FIXME reformatting with new syntax
// TODO FIXME implement arbitrary children matcher
// TODO FIXME allow calling subtemplates inside of a html_extractor?
// TODO FIXME implement else
// TODO FIXME  => a = statement_evaluating_to_unit
// html_handler = html_handler.skip_any_comment(); probably a special case. maybe allow <!-- variable -->
#[derive(Debug)]
struct HtmlIf {
    conditional: Expr,
    body: HtmlCommands,
    variable: Ident,
    result_expr: Expr,
}

impl Parse for HtmlIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        let conditional = input.parse()?;
        let body_parse_buffer;
        let _brace_token = braced!(body_parse_buffer in input);
        let body = body_parse_buffer.parse()?;
        input.parse::<Token![=]>()?;
        input.parse::<Token![>]>()?;
        let variable = input.parse()?;
        input.parse::<Token![=]>()?;
        let result_expr = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(Self { conditional, body, variable, result_expr })
    }
}

#[derive(Debug)]
struct HtmlWhile {
    conditional: Expr,
    body: HtmlCommands,
    variable: Ident,
    result_expr: Expr,
}

impl Parse for HtmlWhile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![while]>()?;
        let conditional = input.parse()?;
        let body_parse_buffer;
        let _brace_token = braced!(body_parse_buffer in input);
        let body = body_parse_buffer.parse()?;
        input.parse::<Token![=]>()?;
        input.parse::<Token![>]>()?;
        let variable = input.parse()?;
        input.parse::<Token![=]>()?;
        let result_expr = Expr::parse_with_earlier_boundary_rule(input)?;
        input.parse::<Token![;]>()?;
        Ok(Self { conditional, body, variable, result_expr })
    }
}

fn convert_commands(commands: &HtmlCommands) -> Vec<TokenStream> {
    commands
        .commands
        .iter()
        .map(|command| match command {
            HtmlCommand::ElementOpen(input) => {
                let tag = input.element.to_string();

                let attributes = input.attributes.iter().map(|iter| {
                    let name = iter
                        .ident
                        .pairs()
                        .map(|p| {
                            p.value().to_string()
                                + match p.punct() {
                                    Some(DashOrColon::Colon) => ":",
                                    Some(DashOrColon::Dash) => "-",
                                    None => "",
                                }
                        })
                        .join("");
                    let value = &iter.value;
                    match value {
                        StringLiteralOrVariable::Literal(lit_str) => {
                            quote_spanned! {lit_str.span()=>
                                #[allow(unused_mut)]
                                let mut html_handler = html_handler.attribute(#name, #lit_str);
                            }
                        }
                        StringLiteralOrVariable::Expression(expr) => {
                            quote_spanned! {expr.span()=>
                                let tmp_internal_html_extractor_proc_macro: &str = #expr;
                                #[allow(unused_mut)]
                                let mut html_handler = html_handler.attribute(#name, tmp_internal_html_extractor_proc_macro);
                            }
                        }
                        StringLiteralOrVariable::Variable(ident) => {
                            quote_spanned! {ident.span()=>
                                #[allow(unused_mut)]
                                let (mut html_handler, #ident) = html_handler.attribute_value(#name);
                            }
                        }
                    }
                });

                let open = quote_spanned! {input.element.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.next_child_tag_open_start(#tag);
                };

                let close = quote_spanned! {input.open_end.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.tag_open_end();
                };

                quote! {
                    #open
                    #(
                        #attributes
                    )*
                    #close
                }
            }
            HtmlCommand::Whitespace(html_whitespace) => {
                quote_spanned! {html_whitespace.underscore.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.skip_whitespace();
                }
            }
            HtmlCommand::ElementClose(html_element_close) => {
                let name = html_element_close.element.to_string();
                quote_spanned! {html_element_close.element.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.close_element(#name);
                }
            }
            HtmlCommand::Comment(html_comment) => {
                let comment = &html_comment.comment;
                quote_spanned! {html_comment.comment.span()=>
                    #[allow(unused_mut)]
                    let mut html_handler = html_handler.skip_comment(#comment);
                }
            }
            HtmlCommand::Text(html_text) => match html_text {
                StringLiteralOrVariable::Literal(lit_str) => {
                    quote_spanned! {lit_str.span()=>
                        #[allow(unused_mut)]
                        let mut html_handler = html_handler.skip_text(#lit_str);
                    }
                }
                StringLiteralOrVariable::Expression(expr) => {
                    quote_spanned! {expr.span()=>
                        #[allow(unused_mut)]
                        let mut html_handler = html_handler.skip_text(#expr);
                    }
                }
                StringLiteralOrVariable::Variable(ident) => {
                    quote_spanned! {ident.span()=>
                        #[allow(unused_mut)]
                        let (mut html_handler, #ident) = html_handler.text();
                    }
                }
            },
            HtmlCommand::If(html_if) => {
                let conditional = &html_if.conditional;
                let body = convert_commands(&html_if.body);
                let variable = &html_if.variable;
                let result_expr = &html_if.result_expr;
                let temp_var = Ident::new("temp_var", Span::mixed_site());
                quote! {
                    let #temp_var;
                    (html_handler, #temp_var) = if (#conditional) {
                        #(#body)*
                        (html_handler, Some(#result_expr))
                    } else {
                        (html_handler, None)
                    };
                    let #variable = #temp_var;
                }
            }
            HtmlCommand::While(html_while) => {
                let conditional = &html_while.conditional;
                let body = convert_commands(&html_while.body);
                let variable = &html_while.variable;
                let result_expr = &html_while.result_expr;
                let temp_vec = Ident::new("temp_vec", Span::mixed_site());
                quote! {
                    let mut #temp_vec = Vec::new();
                    while (#conditional) {
                        html_handler = {
                            let (html_handler, tmp) = {
                                #(#body)*

                                (html_handler, #result_expr)
                            };
                            #temp_vec.push(tmp);
                            html_handler
                        };
                    }
                    let mut #variable = #temp_vec;
                }
            }
        })
        .collect()
}

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as HtmlCommands);

    let expanded = convert_commands(&input);
    let result = quote! {
        #(#expanded)*
    };

    proc_macro::TokenStream::from(result)
}
