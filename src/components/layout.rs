use leptos::*;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="relative isolate px-6 pt-14 lg:px-8">
            <div class="mx-auto max-w-2xl py-16 sm:py-24 lg:py-28">
                <div class="text-center">
                    <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl">
                        "Hi there, 👋"
                    </h1>
                    <p class="mt-6 text-lg leading-8 text-gray-600">
                        "Welcome to sierra!"
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let nav_items = [("About", "/about"), ("Posts", "/posts"), ("Tags", "/tags")];

    view! {
        <header class="relative z-50">
            <nav
                class="mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8"
                aria-label="Global"
            >
                <a href="/" class="-m-1.5 p-1.5">
                    <span class="sr-only">"Home"</span>
                </a>
                <div class="flex gap-x-6 lg:gap-x-12">
                    {nav_items
                        .iter()
                        .map(|(name, href)| {
                            view! {
                                <a
                                    class="text-sm font-semibold leading-6 text-gray-900"
                                    href=href.to_string()
                                >
                                    {name.to_string()}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </nav>
        </header>
    }
}