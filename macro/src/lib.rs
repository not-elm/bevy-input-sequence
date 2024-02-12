#![doc(html_root_url = "https://docs.rs/keyseq_macro/0.1.0")]
#![doc = include_str!("../README.md")]
extern crate proc_macro;
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree, Span, Literal};
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use std::borrow::Cow;

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "bevy")]
mod bevy;

/// Short hand notation describes a physical key chord as `(modifiers: u8,
/// key_code: &str)`.
///
/// Specify a key and any modifiers.
///
/// ```
/// # use keyseq_macro::pkey;
/// assert_eq!(pkey!(A), (0, "A"));
/// assert_eq!(pkey!(shift-A), (1, "A"));
/// assert_eq!(pkey!(ctrl-A), (2, "A"));
/// assert_eq!(pkey!(alt-A), (4, "A"));
/// assert_eq!(pkey!(super-A), (8, "A"));
/// assert_eq!(pkey!(alt-ctrl-;), (6, "Semicolon"));
/// assert_eq!(pkey!(1), (0, "Key1"));
/// assert_eq!(pkey!(alt-1), (4, "Key1"));
/// ```
///
/// More than one key will cause a panic at compile-time. Use keyseq! for that.
///
/// ```compile_fail
/// fn too_many_keys() {
///     let _ = pkey!(A B);
/// }
/// ```
///
/// This macro does not ensure the key names exist but the compiler will.
///
#[cfg_attr(feature = "bevy", doc = r##"
```compile_fail
use keyseq_macro::bevy_pkey as pkey;
use bevy::prelude::KeyCode;
let (mods, key) = pkey!(alt-NoSuchKey); // KeyCode::NoSuchKey does not exist.
```
"##)]
#[proc_macro_error]
#[proc_macro]
pub fn pkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = read_key_chord(input.into(), modifiers_id, get_pkey);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

/// Short hand notation describes a physical key chord as `(modifiers:
/// u8, key: `[bevy::prelude::KeyCode][keycode]`)`.
///
/// [keycode]: https://docs.rs/bevy/latest/bevy/prelude/enum.KeyCode.html
#[cfg_attr(feature = "bevy", doc = r##"
```
use keyseq_macro::bevy_pkey as pkey;
use bevy::prelude::KeyCode;
assert_eq!(pkey!(A), (0, KeyCode::A));
assert_eq!(pkey!(shift-A), (1, KeyCode::A));
assert_eq!(pkey!(ctrl-A), (2, KeyCode::A));
assert_eq!(pkey!(alt-A), (4, KeyCode::A));
assert_eq!(pkey!(super-A), (8, KeyCode::A));
assert_eq!(pkey!(shift-ctrl-A), (3, KeyCode::A));
assert_eq!(pkey!(alt-ctrl-;), (6, KeyCode::Semicolon));
assert_eq!(pkey!(alt-ctrl-Semicolon), (6, KeyCode::Semicolon));
assert_eq!(pkey!(1), (0, KeyCode::Key1));
assert_eq!(pkey!(alt-1), (4, KeyCode::Key1));
```
"##)]
#[cfg(feature = "bevy")]
#[proc_macro_error]
#[proc_macro]
pub fn bevy_pkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = read_key_chord(input.into(), modifiers_id, bevy::get_pkey);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

/// Short hand notation describes a physical key chord as `(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key_code: `[winit::keyboard::KeyCode][keycode]`)`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [keycode]: https://docs.rs/winit/latest/winit/keyboard/enum.KeyCode.html
#[cfg_attr(feature = "winit", doc = r##"
```
use keyseq_macro::winit_pkey as pkey;
use winit::keyboard::{ModifiersState, KeyCode};
assert_eq!(pkey!(A), (ModifiersState::empty(), KeyCode::KeyA));
```
"##)]
#[cfg(feature = "winit")]
#[proc_macro_error]
#[proc_macro]
pub fn winit_pkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = read_key_chord(input.into(), winit::modifiers_id, winit::get_pkey);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

/// Short hand notation describes a logical key chord as `(modifiers: u8,
/// key_code: &str)`.
///
/// Specify a key and any modifiers.
///
/// ```
/// # use keyseq_macro::key;
/// assert_eq!(key!(a), (0, "a"));
/// assert_eq!(key!(A), (0, "A"));
/// assert_eq!(key!(shift-A), (1, "A"));
/// assert_eq!(key!(ctrl-A), (2, "A"));
/// assert_eq!(key!(alt-A), (4, "A"));
/// assert_eq!(key!(super-A), (8, "A"));
/// assert_eq!(key!(alt-ctrl-;), (6, ";"));
/// assert_eq!(key!(1), (0, "1"));
/// assert_eq!(key!(alt-1), (4, "1"));
/// ```
///
/// More than one key will cause a panic at compile-time. Use keyseq! for that.
///
/// ```compile_fail
/// fn too_many_keys() {
///     let _ = key!(A B);
/// }
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn key(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = read_key_chord(input.into(), modifiers_id, get_key);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

/// Short hand notation describes a logical key chord as `(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key: `[winit::keyboard::Key][key]`)`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [key]: https://docs.rs/winit/latest/winit/keyboard/enum.Key.html
#[cfg_attr(feature = "winit", doc = r##"
```
use keyseq_macro::winit_key as key;
use winit::keyboard::{ModifiersState, Key};
assert_eq!(key!(a), (ModifiersState::empty(), Key::Character('a')));
assert_eq!(key!(A), (ModifiersState::empty(), Key::Character('A')));
assert_eq!(key!(shift-A), (ModifiersState::SHIFT, Key::Character('A')));
assert_eq!(key!(shift-a), (ModifiersState::SHIFT, Key::Character('a')));
assert_eq!(key!(ctrl-A), (ModifiersState::CONTROL, Key::Character('A')));
assert_eq!(key!(alt-A), (ModifiersState::ALT, Key::Character('A')));
assert_eq!(key!(super-A), (ModifiersState::SUPER, Key::Character('A')));
assert_eq!(key!(alt-ctrl-;), (ModifiersState::ALT | ModifiersState::CONTROL, Key::Character(';')));
assert_eq!(key!(1), (ModifiersState::empty(), Key::Character('1')));
assert_eq!(key!(!), (ModifiersState::empty(), Key::Character('!')));
```

```
use keyseq_macro::winit_pkey as pkey;
use winit::keyboard::{ModifiersState, KeyCode};
assert_eq!(pkey!(A), (ModifiersState::empty(), KeyCode::KeyA));
```

"##)]

#[cfg_attr(feature = "winit", doc = r##"
```
use keyseq_macro::winit_key as key;
use winit::keyboard::{ModifiersState, Key};
assert_eq!(key!(;), (ModifiersState::empty(), Key::Character(';')));
assert_eq!(key!(ctrl-;), (ModifiersState::CONTROL, Key::Character(';')));
```

This does have a limitation though because the macro does not do reverse look
ups from character to name.

```compile_fail
# use keyseq_macro::winit_key as key;
use winit::keyboard::{ModifiersState, Key};
assert_eq!(key!(ctrl-Semicolon), (ModifiersState::CONTROL, Key::Character(';')));
```
"##)]
#[cfg(feature = "winit")]
#[proc_macro_error]
#[proc_macro]
pub fn winit_key(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = read_key_chord(input.into(), winit::modifiers_id, winit::get_key);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

// #[cfg(feature = "bevy")]
// #[proc_macro_error]
// #[proc_macro]
// pub fn bevy_lkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let (result, leftover) = read_key_chord(input.into(), bevy::modifiers_id, bevy::get_key);
//     if !leftover.is_empty() {
//         abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
//     }
//     result.into()
// }

/// Short hand notation describes a sequence of logical key chords as
/// `[(modifiers: u8, key_code: &str)]`.
///
///
/// ```
/// use keyseq_macro::keyseq;
/// assert_eq!(keyseq!(A B), [(0, "A"), (0, "B")]);
/// assert_eq!(keyseq!(shift-A ctrl-B), [(1, "A"), (2, "B")]);
/// ```
///
/// When no features are enabled, there are no smarts to check whether a key is real
/// or not.
///
/// ```
/// # use keyseq_macro::keyseq;
/// assert_eq!(keyseq!(A NoSuchKey), [(0, "A"), (0, "NoSuchKey")]);
/// ```
///
#[proc_macro_error]
#[proc_macro]
pub fn keyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let keys = read_key_chords(input.into(), modifiers_id, get_key);
    quote! {
        [#(#keys),*]
    }
    .into()
}

/// Short hand notation describes a sequence of physical key chord as `[(modifiers:
/// u8, key: `[bevy::prelude::KeyCode][keycode]`)]`.
///
/// [keycode]: https://docs.rs/bevy/latest/bevy/prelude/enum.KeyCode.html
#[cfg(feature = "bevy")]
#[proc_macro_error]
#[proc_macro]
pub fn bevy_pkeyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let keys = read_key_chords(input.into(), modifiers_id, bevy::get_pkey);
    quote! {
        [#(#keys),*]
    }
    .into()
}

/// Short hand notation describes a sequence of physical key chord as `[(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key_code: `[winit::keyboard::KeyCode][keycode]`)]`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [keycode]: https://docs.rs/winit/latest/winit/keyboard/enum.KeyCode.html
#[cfg(feature = "winit")]
#[proc_macro_error]
#[proc_macro]
pub fn winit_pkeyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let keys = read_key_chords(input.into(), winit::modifiers_id, winit::get_pkey);
    quote! {
        [#(#keys),*]
    }
    .into()
}

/// Short hand notation describes a sequence of logical key chord as `[(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key: `[winit::keyboard::Key][key]`)]`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [key]: https://docs.rs/winit/latest/winit/keyboard/enum.Key.html
#[cfg(feature = "winit")]
#[proc_macro_error]
#[proc_macro]
pub fn winit_keyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let keys = read_key_chords(input.into(), winit::modifiers_id, winit::get_key);
    quote! {
        [#(#keys),*]
    }
    .into()
}

fn read_key_chords<F,G>(input: TokenStream, modifiers_id: F, get_key: G) -> Vec<TokenStream>
    where F: Fn(Modifier) -> TokenStream,
    G: Fn(TokenTree) -> Option<TokenStream>
{
    let mut input: TokenStream = input.into();
    let mut keys = vec![];

    loop {
        let (result, leftover) = read_key_chord(input, &modifiers_id, &get_key);
        keys.push(result);
        if leftover.is_empty() {
            break;
        }
        input = leftover;
    }
    keys
}

fn key_code_path(id: Ident) -> TokenStream {
    let x = format!("{}", id);
    let s = proc_macro2::Literal::string(&x);
    quote!{ #s }
}

fn get_pkey(tree: TokenTree) -> Option<TokenStream> {
    match tree {
        TokenTree::Literal(ref literal) => {
            let x = literal.span().source_text().unwrap();
            if x.len() == 1 && x.parse::<u8>().is_ok() {
                eprintln!("got numeric literal {:?}", x);
                Some(Ident::new(&format!("Key{x}"), Span::call_site()))
                // Some(Ident::new("Keyx", Span::call_site()))
            } else {
                let name = match x.as_str() {
                    "'\\''" => Some("Apostrophe"),
                    "'`'" => Some("Grave"),
                    "'\\\\'" => Some("Backslash"),
                    _ => todo!("literal char {x} {:?}", literal),
                };
                name.map(|x| Ident::new(x, Span::call_site()))
            }
        }
        TokenTree::Punct(ref punct) => {
            let name: Option<&str> = match punct.as_char() {
                ';' => Some("Semicolon"),
                ':' => {
                    // TODO: `ctrl-:` Can't be entered on a US ANSI
                    // keyboard only `shift-;` can. Make docs clear this
                    // is the key and not the symbol?

                    // add_shift = true;
                    // Some("Semicolon")
                    Some("Colon")
                }
                ',' => Some("Comma"),
                '.' => Some("Period"),
                '^' => Some("Caret"),
                '=' => Some("Equals"),
                '/' => Some("Slash"),
                '-' => Some("Minus"),
                '*' => Some("Asterisk"),
                '+' => Some("Plus"),
                '@' => Some("At"),
                // _ => None
                _ => todo!("punct {:?}", punct),
            };
            name.map(|n| Ident::new(n, punct.span()))
        }
        TokenTree::Ident(ref ident) => {
            let label = ident.span().source_text().unwrap();
            if label.len() == 1 {
                let name: Option<Cow<'static, str>> = match label.chars().next().unwrap() {
                    'A'..='Z' => {
                        Some(label.into())
                    }
                    x @ 'a'..='z' => {
                        abort!(x, "Use uppercase key names");
                        // let s = x.to_ascii_uppercase().to_string();
                        // Some(s.into())
                    }
                    _ => todo!("ident {:?}", ident),
                };
                name.as_ref().map(|n| Ident::new(n, ident.span()))
            } else {
                Some(ident.clone())
            }
        }
        _ => None,
    }.map(key_code_path)
}

enum Modifier {
    // Use same order as winit.
    None = 0,
    Shift = 1,
    Control = 2,
    Alt = 3,
    Super = 4,
}

impl Modifier {
    #[allow(dead_code)]
    fn to_tokens(&self) -> TokenStream {
        match self {
            Modifier::None => quote! { empty() },
            Modifier::Shift => quote! { SHIFT },
            Modifier::Control => quote! { CONTROL },
            Modifier::Alt => quote! { ALT },
            Modifier::Super => quote! { SUPER },
        }
    }
}

#[allow(unused_variables)]
fn modifiers_id(modifier: Modifier) -> TokenStream {
    let mut number = modifier as u8;
    if number != 0 {
        number = 1 << (number - 1);

        // This is the bitflag scheme that winit's ModifiersState uses:
        // number = 1 << (number - 1) * 3;
    }
    let x = proc_macro2::Literal::u8_suffixed(number);
    quote! { #x }
}


fn get_key(tree: TokenTree) -> Option<TokenStream> {
    get_key_raw(tree).map(|r| match r {
        Ok(c) => {
            let l = Literal::string(&c.to_string());
            quote! { #l }
        },
        Err(cow) => {
            let l = Literal::string(&cow);
            quote! { #l }
        }
    })
}

fn get_key_raw(tree: TokenTree) -> Option<Result<char, Cow<'static, str>>> {
    match tree {
        TokenTree::Literal(ref literal) => {
            let x = literal.span().source_text().unwrap();
            if x.len() == 1 {
                Some(Ok(x.chars().next().unwrap()))
            } else {
                let name = match x.as_str() {
                    "'\\''" => Some("Apostrophe"),
                    "'`'" => Some("Grave"),
                    "'\\\\'" => Some("Backslash"),
                    _ => todo!("literal char {x} {:?}", literal),
                };
                Some(Err(name.map(|n| n.to_string()).unwrap_or(x).into()))
            }
        }
        TokenTree::Punct(ref punct) => {
            Some(Ok(punct.as_char()))
        }
        TokenTree::Ident(ref ident) => {
            let label = ident.span().source_text().unwrap();
            if label.len() == 1 {
                Some(Ok(label.chars().next().unwrap()))
            } else {
                Some(Err(label.into()))
            }
        }
        _ => None,
    }
}

fn read_modifiers<F: Fn(Modifier) -> TokenStream>(input: TokenStream, modifiers_id: F) -> (TokenStream, TokenStream) {
    let mut r = TokenStream::new();
    let mut i = input.into_iter().peekable();
    let mut last_tree = None;

    fn is_dash(tree: &TokenTree) -> bool {
        match tree {
            TokenTree::Punct(ref punct) => punct.as_char() == '-',
            _ => false,
        }
    }

    while let Some(tree) = i.next() {
        if i.peek().is_none() || (!is_dash(&tree) && !is_dash(i.peek().unwrap())) {
            last_tree = Some(tree);
            break;
        } else {
            let replacement = match tree {
                TokenTree::Ident(ref ident) => match ident.span().source_text().unwrap().as_str() {
                    "shift" => Some(TokenTree::Group(Group::new(
                        Delimiter::None,
                        modifiers_id(Modifier::Shift),
                    ))),
                    "ctrl" => Some(TokenTree::Group(Group::new(
                        Delimiter::None,
                        modifiers_id(Modifier::Control),
                    ))),
                    "alt" => Some(TokenTree::Group(Group::new(
                        Delimiter::None,
                        modifiers_id(Modifier::Alt),
                    ))),
                    "super" => Some(TokenTree::Group(Group::new(
                        Delimiter::None,
                        modifiers_id(Modifier::Super),
                    ))),
                    _ => None,
                },
                TokenTree::Punct(ref punct) => match punct.as_char() {
                    // We could allow + notation too.
                    '-' => Some(TokenTree::Punct(Punct::new('|', Spacing::Alone))),
                    _ => None,
                },
                _ => None,
            };
            r.extend([replacement.unwrap_or(tree)]);
        }
    }
    // This will add an empty to finish the expression:
    //
    //    ctrl-alt-EMPTY -> Control | Alt | EMPTY.
    //
    //  And it will provide a valid Modifier when none have been provided.
    r.extend([modifiers_id(Modifier::None)]);
    (
        r,
        TokenStream::from_iter(last_tree.into_iter().chain(i)),
    )
}

fn read_key<F: Fn(TokenTree) -> Option<TokenStream>>(input: TokenStream, get_key: F) -> (TokenStream, TokenStream) {
    let mut i = input.into_iter();
    let tree = i.next().expect("No token tree");
    let key = get_key(tree).expect("No logical key found");
    (
        quote! {
            #key
        },
        TokenStream::from_iter(i),
    )
}

fn read_key_chord<F,G>(input: TokenStream, modifiers_id: F, get_key: G) -> (TokenStream, TokenStream)
    where F:Fn(Modifier) -> TokenStream,
    G: Fn(TokenTree) -> Option<TokenStream>
{
    let (mods, input) = read_modifiers(input, modifiers_id);
    let (key, rest) = read_key(input, get_key);
    (
        quote! {
            (#mods, #key)
        },
        rest
    )
}
