use dioxus::prelude::*;

use crate::shared::ui::{
    Badge, BadgeVariant, Button, ButtonVariant, Input, Separator,
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};

const MOCK_INVESTMENTS: &[InvRow] = &[
    InvRow { fund: "Quy Nhon Coastal", client: "Nguyen Family", committed: "$2.4M", called: "$1.8M", irr: "21.3%", status: "Active" },
    InvRow { fund: "Da Nang Premium", client: "Horizon Capital", committed: "$5.1M", called: "$4.6M", irr: "19.7%", status: "Active" },
    InvRow { fund: "Phu Quoc Resort", client: "Green Path Ltd", committed: "$1.8M", called: "$0.4M", irr: "—", status: "Pending" },
    InvRow { fund: "Hoi An Heritage", client: "Le & Partners", committed: "$900K", called: "$900K", irr: "16.2%", status: "Active" },
    InvRow { fund: "Nha Trang Bay", client: "Sunrise Fund", committed: "$3.2M", called: "$3.2M", irr: "18.9%", status: "Closed" },
    InvRow { fund: "Mui Ne Dunes", client: "Tran Group", committed: "$1.2M", called: "$0.3M", irr: "—", status: "Pending" },
];
#[component]
pub fn Investments() -> Element {
    rsx! {
        div { class: "space-y-6",
            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-2xl font-bold text-foreground", "Investments" }
                    p { class: "text-sm text-muted-foreground mt-1", "Fund positions & returns" }
                }
                Button { variant: ButtonVariant::Default, "New Position" }
            }

            Separator {}

            div { class: "flex gap-3",
                Input { placeholder: "Search investments…", class: "max-w-xs" }
            }

            Table {
                TableHeader {
                    TableRow {
                        TableHead { "Fund" }
                        TableHead { "Client" }
                        TableHead { "Committed" }
                        TableHead { "Called" }
                        TableHead { "IRR" }
                        TableHead { "Status" }
                    }
                }
                TableBody {
                    for inv in MOCK_INVESTMENTS {
                        TableRow { key: "{inv.fund}",
                            TableCell { class: "font-medium", "{inv.fund}" }
                            TableCell { class: "text-muted-foreground", "{inv.client}" }
                            TableCell { "{inv.committed}" }
                            TableCell { class: "text-muted-foreground", "{inv.called}" }
                            TableCell { class: "text-main-accent-t2", "{inv.irr}" }
                            TableCell {
                                Badge { variant: status_variant(inv.status), "{inv.status}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct InvRow {
    fund: &'static str,
    client: &'static str,
    committed: &'static str,
    called: &'static str,
    irr: &'static str,
    status: &'static str,
}


fn status_variant(status: &str) -> BadgeVariant {
    match status {
        "Active" => BadgeVariant::Success,
        "Pending" => BadgeVariant::Secondary,
        "Closed" => BadgeVariant::Outline,
        _ => BadgeVariant::Default,
    }
}
