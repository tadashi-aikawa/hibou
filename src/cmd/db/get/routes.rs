use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use strum::VariantNames;

use crate::app::route::{RouteService, RouteServiceDb};
use crate::io::Format;
use crate::{external, io};

#[derive(Clap, Debug)]
pub struct Opts {
    /// 読み込むデータベースファイルのパス
    #[clap(short, long, parse(from_os_str), default_value = "gtfs.db")]
    database: PathBuf,
    /// 出力フォーマット
    #[clap(short, long, default_value = "csv", possible_values(Format::VARIANTS))]
    format: Format,
}

pub fn run(op: &Opts) -> Result<()> {
    let gtfs = external::gtfsdb::GtfsDb::new(&op.database)?;
    let routes = RouteServiceDb::new(gtfs).fetch()?;
    io::write(&routes, &op.format)?;
    Ok(())
}
