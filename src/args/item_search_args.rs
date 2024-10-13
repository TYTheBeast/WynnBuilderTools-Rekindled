use crate::Apparel;
use casey::lower;
use clap::{Parser, ValueEnum};
use std::fmt::Display;
use std::str::FromStr;

use super::generate_sort_by::generate_sort_by;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct ItemSearchArgs {
    /// Apparel type
    #[arg(short, long)]
    pub r#type: Option<r#Type>,

    /// A limit on the number of results, auto-inflated if the last item has the same value as multiple items
    #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u32).range(1..))]
    pub limit: u32,

    /// Order the results in ascending or descending order
    #[arg(short, long, default_value_t = OrderBy::Desc)]
    pub order_by: OrderBy,

    /// Sort the results by a specific field
    #[arg(short, long, num_args = 1..)]
    pub sort_by: Vec<SortAndFilterBy>,

    /// Minimum values for various attributes (format: attribute=value)
    #[arg(long = "min", value_parser = parse_key_val_sort_by)]
    pub min_values: Vec<(SortAndFilterBy, i32)>,

    /// Maximum values for various attributes (format: attribute=value)
    #[arg(long = "max", value_parser = parse_key_val_sort_by)]
    pub max_values: Vec<(SortAndFilterBy, i32)>,
}

fn parse_key_val_sort_by(s: &str) -> Result<(SortAndFilterBy, i32), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    let key = s[..pos].parse::<SortAndFilterBy>()?;
    let value = s[pos + 1..]
        .parse()
        .map_err(|e| format!("invalid value: {}; an integer is required", e))?;
    Ok((key, value))
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum r#Type {
    Helmets,
    ChestPlate,
    Leggings,
    Boots,
    Ring,
    Bracelet,
    Necklace,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OrderBy {
    /// Sort the results in ascending order, arrange them from smallest to largest
    Asc,
    /// Sort the results in descending order, arrange them from largest to smallest
    Desc,
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBy::Asc => write!(f, "asc"),
            OrderBy::Desc => write!(f, "desc"),
        }
    }
}

generate_sort_by! { item =>
    Lvl => item.lvl,
    Hp => item.hp,
    Hpb => item.hp_bonus_max,
    HprRaw => item.stat_max.hpr_raw() as i32,
    HprPct => item.stat_max.hpr_pct() as i32,
    SPAdd => item.add.all() as i32,
    SPReq => item.req.all() as i32,
    SDRaw => item.stat_max.sd_raw() as i32,
    SDPct => item.stat_max.sd_pct() as i32,
    Mr => item.stat_max.mr() as i32,
    Spd => item.stat_max.spd() as i32,
    Ls => item.stat_max.ls() as i32,
    ExpB => item.max_exp_bonus,
    Ndmg => item.dam_pct_max.n() as i32,
    Edmg => item.dam_pct_max.e() as i32,
    Tdmg => item.dam_pct_max.t() as i32,
    Wdmg => item.dam_pct_max.w() as i32,
    Fdmg => item.dam_pct_max.f() as i32,
    Admg => item.dam_pct_max.a() as i32,
}
