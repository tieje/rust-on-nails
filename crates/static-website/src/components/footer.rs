use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "mt-12 flex flex-row justify-around bg-neutral text-neutral-content p-10",
            nav {
                h6 {
                    class: "footer-title",
                    "Resources"
                }
                a {
                    href: crate::routes::blog::Index {}.to_string(),
                    class: "block link-hover",
                    "Blog"
                }
            }
            nav {
                h6 {
                    class: "footer-title",
                    "Company"
                }
                a {
                    class: "block link-hover",
                    "About Us"
                }
            }
            nav {
                h6 {
                    class: "footer-title",
                    "Legal"
                }
                a {
                    href: crate::routes::marketing::Terms {}.to_string(),
                    class: "block link-hover",
                    "Terms of Use"
                }
                a {
                    href: crate::routes::marketing::Privacy {}.to_string(),
                    class: "block link-hover",
                    "Privacy Policy"
                }
            }
        }
    }
}
