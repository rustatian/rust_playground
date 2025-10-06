use self::wasi::nn::{
    graph::{ExecutionTarget, Graph, GraphEncoding, load},
    tensor::{Tensor, TensorType},
};
use image::{ImageReader, imageops::Triangle};
use ndarray::Array;
use std::io::BufRead;
use std::{env, fs};

wit_bindgen::generate!({
    path: "wit/wasi-nn.wit",
    world: "ml",
});

fn main() {}
