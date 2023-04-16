use yew::prelude::*;
use crate::{contexts::theme::{THEMES, use_theme}, components::{theme_card::ThemeCard, header::Header}};

#[function_component(Settings)]
pub fn settings() -> Html {
    let theme = use_theme().state();
    let mut theme_btns_html: Vec<Html> = Vec::new(); 
    
    for theme in THEMES {
        let att = AttrValue::from(theme.clone());
        let html = html!{ <ThemeCard name={att} /> };
        theme_btns_html.push(html);
    };

    html! {
        <div data-theme={theme} class="select-none">
            <Header />
            <div class="flex justify-center h-[calc(100vh-64px)] bg-base-200">
                <div class="flex flex-col
                w-screen
                2xl:w-[50vw] xl:w-[50vw]
                lg:w-[85vw] md:w-[90vw]
                sm:w-[95vw] my-[3vh] px-6
                overscroll-contain overflow-visible overflow-y-auto">
                    <div class="flex flex-col gap-3">
                        <h1 class="font-display text-3xl cursor-default">{"Theme"}</h1>
                        <div class="flex flex-wrap flex-row gap-4">
                            { for theme_btns_html }
                        </div>
                    </div>
                    <div class="mt-16">
                        <h1 class="font-display text-3xl cursor-default">{"General"}</h1>
                        <div class="flex flex-col">
                            <div class="divider" />
                            <div class="form-control w-full">
                                <label class="cursor-pointer label">
                                <div class="tooltip tooltip-right" data-tip={"Ubiquity will save your markdown file for you."}>
                                    <span class="font-mono text-2xl">{"Autosave"}</span>
                                </div>
                                <input type="checkbox" class="toggle toggle-primary" checked={true} />
                                </label>
                            </div>
                            <div class="divider" />
                            <div class="form-control w-full">
                                <label class="cursor-pointer label">
                                    <div class="tooltip tooltip-right" data-tip={"Disable the HTML preview of your markdown."}>
                                        <span class="font-mono text-2xl">{"Markdown Preview"}</span>
                                    </div>
                                    <input type="checkbox" class="toggle toggle-primary" checked={true} />
                                </label>
                            </div>  
                        </div>
                    </div>           
                </div>
            </div>
        </div>
    }
}
