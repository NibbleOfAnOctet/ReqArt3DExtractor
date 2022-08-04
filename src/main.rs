mod req_art_3d;
mod dcl;
mod bitmap;

use clap::{Parser};
use req_art_3d::*;

#[derive(Parser)]
#[clap(author="Created by NibbleOfAnOctet", version="v0.0.1", about="A tool for extracting assets from ReqArt3D files used in Requiem: Avenging Angel.")]
struct Args{
    /// Path to ReqArt3D.art/ReqArt3D.dat
    #[clap(value_parser)]
    path_to_reqart: String,

    /// Path to output folder
    #[clap(value_parser)]
    output_folder: String
}

fn main() {
    let _args = Args::parse();
    let mut req_art: ReqArt3D = ReqArt3D::open(_args.path_to_reqart);
    req_art.extract(_args.output_folder);
    
}
