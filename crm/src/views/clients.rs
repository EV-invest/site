use dioxus::prelude::*;

use crate::shared::ui::{
    Badge, BadgeVariant, Button, ButtonVariant, Input, Separator,
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};

const MOCK_CLIENTS: &[ClientRow] = &[
    ClientRow { name: "Nguyen Family Office", client_type: "Family Office", aum: "$12.4M", kyc: "Verified", since: "2022" },
    ClientRow { name: "Horizon Capital", client_type: "Institutional", aum: "$28.7M", kyc: "Verified", since: "2021" },
    ClientRow { name: "Green Path Ltd", client_type: "Corporate", aum: "$4.2M", kyc: "Pending", since: "2024" },
    ClientRow { name: "Le & Partners", client_type: "HNWI", aum: "$6.8M", kyc: "Verified", since: "2023" },
    ClientRow { name: "Sunrise Fund", client_type: "Institutional", aum: "$19.1M", kyc: "Verified", since: "2020" },
    ClientRow { name: "Tran Group", client_type: "Family Office", aum: "$3.5M", kyc: "Review", since: "2024" },
];
#[component]
pub fn Clients() -> Element {
    rsx! {
        div { class: "space-y-6",
            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-2xl font-bold text-foreground", "Clients" }
                    p { class: "text-sm text-muted-foreground mt-1", "Investor registry" }
                }
                Button { variant: ButtonVariant::Default, "Add Client" }
            }

            Separator {}

            div { class: "flex gap-3",
                Input { placeholder: "Search clients…", class: "max-w-xs" }
            }

            Table {
                TableHeader {
                    TableRow {
                        TableHead { "Name" }
                        TableHead { "Type" }
                        TableHead { "AUM" }
                        TableHead { "KYC" }
                        TableHead { "Since" }
                    }
                }
                TableBody {
                    for c in MOCK_CLIENTS {
                        TableRow { key: "{c.name}",
                            TableCell { class: "font-medium", "{c.name}" }
                            TableCell { class: "text-muted-foreground", "{c.client_type}" }
                            TableCell { "{c.aum}" }
                            TableCell {
                                Badge { variant: kyc_variant(c.kyc), "{c.kyc}" }
                            }
                            TableCell { class: "text-muted-foreground", "{c.since}" }
                        }
                    }
                }
            }
        }
    }
}

struct ClientRow {
    name: &'static str,
    client_type: &'static str,
    aum: &'static str,
    kyc: &'static str,
    since: &'static str,
}


fn kyc_variant(kyc: &str) -> BadgeVariant {
    match kyc {
        "Verified" => BadgeVariant::Success,
        "Pending" => BadgeVariant::Secondary,
        "Review" => BadgeVariant::Default,
        _ => BadgeVariant::Outline,
    }
}
