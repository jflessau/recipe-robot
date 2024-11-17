use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vendor {
    Rewe,
    Lidl,
    Penny,
}

impl std::fmt::Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vendor::Rewe => write!(f, "Rewe"),
            Vendor::Lidl => write!(f, "Lidl"),
            Vendor::Penny => write!(f, "Penny"),
        }
    }
}

#[component]
pub fn View() -> impl IntoView {
    let (vendors, set_vendors) = create_signal(vec![
        (Vendor::Rewe, false),
        (Vendor::Lidl, false),
        (Vendor::Penny, false),
    ]);

    let select_vendor = move |new| {
        let mut new_vendors = vendors.get();
        for (vendor, active) in new_vendors.iter_mut() {
            *active = *vendor == new;
        }
        set_vendors(new_vendors);
    };

    view! {
        <div class="w-full flex flex-col gap-6">
            <h2 class="text-xl text-center font-black text-attention">"2. Laden wählen"</h2>

            <div class="flex flex-row justify-center gap-4">
                <For each=vendors key=|vendor| format!("{:?}-{}", vendor.0, vendor.1) let:item>
                    <VendorButton caption=item.0 active=item.1 select_vendor=select_vendor />
                </For>
            </div>
        </div>
    }
}

#[component]
pub fn VendorButton(
    caption: Vendor,
    active: bool,
    select_vendor: impl Fn(Vendor) + 'static,
) -> impl IntoView {
    let button_classes = "bg-bg rounded-lg px-4 py-1 border border-contrast".to_string();
    let c = caption.clone();
    let new_vendor = c.clone();

    view! {
        <button
            class=("bg-info", move || active)
            class=("text-color-inverted", move || active)
            on:click=move |_| {
                select_vendor(new_vendor.clone());
            }
            class=button_classes.clone()
        >
            {caption.to_string()}
        </button>
    }
}
