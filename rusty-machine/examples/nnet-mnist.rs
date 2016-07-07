extern crate rusty_machine;
extern crate rand;

use rand::{random, Closed01};
use std::vec::Vec;

use rusty_machine::learning::nnet::{NeuralNet, BCECriterion};
use rusty_machine::learning::toolkit::regularization::Regularization;
use rusty_machine::learning::optim::grad_desc::StochasticGD;

use rusty_machine::linalg::matrix::Matrix;
use rusty_machine::learning::SupModel;

// MNIST
fn main() {
    println!("MNIST handwritten digit recognition sample:");
    println!("WIP");
}
