use dioxus::prelude::*;

use crate::shared::ui::{
    Badge, BadgeVariant, Card, CardContent, CardDescription, CardHeader, CardTitle,
    Separator, Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};

const MOCK_INVESTMENTS: &[InvestmentRow] = &[
    InvestmentRow { fund: "Quy Nhon Coastal", client: "Nguyen Family", amount: "$2.4M", status: "Active", date: "2024-11-01" },
    InvestmentRow { fund: "Da Nang Premium", client: "Horizon Capital", amount: "$5.1M", status: "Active", date: "2024-10-15" },
    InvestmentRow { fund: "Phu Quoc Resort", client: "Green Path Ltd", amount: "$1.8M", status: "Pending", date: "2024-10-03" },
    InvestmentRow { fund: "Hoi An Heritage", client: "Le & Partners", amount: "$900K", status: "Active", date: "2024-09-20" },
    InvestmentRow { fund: "Nha Trang Bay", client: "Sunrise Fund", amount: "$3.2M", status: "Closed", date: "2024-08-14" },
];
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "space-y-6",
            div {
                h1 { class: "text-2xl font-bold text-foreground", "Dashboard" }
                p { class: "text-sm text-muted-foreground mt-1", "Portfolio overview" }
            }

            Separator {}

            div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4",
                KpiCard {
                    title: "Total AUM",
                    value: "$124.5M",
                    delta: "+12.4% YTD",
                    value_class: "text-main-accent-t3",
                }
                KpiCard {
                    title: "Net IRR",
                    value: "18.7%",
                    delta: "Target 15%",
                    value_class: "text-main-accent-t2",
                }
                KpiCard {
                    title: "Active Clients",
                    value: "47",
                    delta: "3 pending KYC",
                    value_class: "text-foreground",
                }
            }

            Card {
                CardHeader {
                    CardTitle { "Recent Investments" }
                    CardDescription { "Last 5 fund transactions" }
                }
                CardContent {
                    Table {
                        TableHeader {
                            TableRow {
                                TableHead { "Fund" }
                                TableHead { "Client" }
                                TableHead { "Amount" }
                                TableHead { "Status" }
                                TableHead { "Date" }
                            }
                        }
                        TableBody {
                            for row in MOCK_INVESTMENTS {
                                TableRow { key: "{row.fund}",
                                    TableCell { class: "font-medium", "{row.fund}" }
                                    TableCell { "{row.client}" }
                                    TableCell { "{row.amount}" }
                                    TableCell {
                                        Badge {
                                            variant: status_variant(row.status),
                                            "{row.status}"
                                        }
                                    }
                                    TableCell { class: "text-muted-foreground", "{row.date}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn KpiCard(title: String, value: String, delta: String, value_class: String) -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { class: "text-sm font-medium text-muted-foreground", "{title}" }
            }
            CardContent {
                p { class: "text-3xl font-bold {value_class}", "{value}" }
                p { class: "text-xs text-muted-foreground mt-1", "{delta}" }
            }
        }
    }
}

struct InvestmentRow {
    fund: &'static str,
    client: &'static str,
    amount: &'static str,
    status: &'static str,
    date: &'static str,
}


fn status_variant(status: &str) -> BadgeVariant {
    match status {
        "Active" => BadgeVariant::Success,
        "Pending" => BadgeVariant::Secondary,
        "Closed" => BadgeVariant::Outline,
        _ => BadgeVariant::Default,
    }
}
