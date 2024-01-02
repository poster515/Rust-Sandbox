
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex, Condvar};

use num_complex::Complex;

pub type ReturnPair = (Arc<Mutex<bool>>, Arc<Condvar>);
pub type DataBuffer = Arc<Mutex<Vec<Complex<f64>>>>;
pub type DspBufferVector = Arc<Mutex<Vec<DataBuffer>>>;


#[derive(Default)]
pub struct HalfPipe {
    pub data: DataBuffer,
    pub data_ready: Arc<Mutex<bool>>,
    pub cond_var: Arc<Condvar>,
}

impl HalfPipe {
    pub fn new(i_vec: DataBuffer, i_bool: Arc<Mutex<bool>>, i_cv: Arc<Condvar>) -> HalfPipe {
        HalfPipe {
            data: i_vec,
            data_ready: i_bool,
            cond_var: i_cv,
        }
    }
    pub fn empty() -> HalfPipe{
        HalfPipe {
            data: Arc::new(Mutex::new(Vec::new())),
            data_ready: Arc::new(Mutex::new(false)),
            cond_var: Arc::new(Condvar::new())
        }
    }
    pub fn clone(&self) -> HalfPipe {
        HalfPipe {
            data: Arc::clone(&self.data),
            data_ready: Arc::clone(&self.data_ready),
            cond_var: Arc::clone(&self.cond_var)
        }
    }
}


pub trait WrapperTraits {
    fn new(i_vec: DataBuffer, i_bool: Arc<Mutex<bool>>, i_cv: Arc<Condvar>, ch: usize) -> DataPipe;
    fn clone(&self) -> DataPipe;
    fn get_output_data_vec(&self) -> DataBuffer;
    fn get_input_data_vec(&self) -> DataBuffer;
    fn get_output_halfpipe(&self) -> HalfPipe;
    fn get_input_halfpipe(&self) -> HalfPipe;
    fn wait_for_start(&mut self) -> ();
    fn end_of_processing(&mut self) -> ();
}

pub struct DataPipe {
    pub input: HalfPipe,
    pub output: HalfPipe,
    pub channel: usize
}

impl WrapperTraits for DataPipe {

    fn new(i_vec: DataBuffer, i_bool: Arc<Mutex<bool>>, i_cv: Arc<Condvar>, ch: usize) -> DataPipe {
        DataPipe {
            input: HalfPipe::new(i_vec, i_bool, i_cv),
            output: HalfPipe::empty(),
            channel: ch
        }
    }

    fn clone(&self) -> DataPipe {
        DataPipe {
            input: self.input.clone(),
            output: self.output.clone(),
            channel: self.channel
        }
    }

    fn get_output_data_vec(&self) -> DataBuffer {
        Arc::clone(&self.output.data)
    }

    fn get_input_data_vec(&self) -> DataBuffer {
        Arc::clone(&self.input.data)
    }

    fn get_output_halfpipe(&self) -> HalfPipe {
        self.output.clone()
    }

    fn get_input_halfpipe(&self) -> HalfPipe {
        self.input.clone()
    }

    fn wait_for_start(&mut self) -> () {
        // not done, make sure we mark ourselves as 'incomplete'
        let mut finished = self.output.data_ready.lock().unwrap();
        *finished = false;

        // now wait for prev component to be done
        let mut prev_finished = self.input.data_ready.lock().unwrap();
        while !*prev_finished {
            prev_finished = self.input.cond_var.wait(prev_finished).unwrap();
        }
        println!("{} got start signal!!", self.channel);
    }

    fn end_of_processing(&mut self) -> () {
        // mark ourselves as done and notify next listener
        println!("{} marking itself as 'done'...", self.channel);
        let mut finished = self.output.data_ready.lock().unwrap();
        *finished = true;
        self.output.cond_var.notify_one();
    }
}

pub type DspChannels = Arc<Mutex<Vec<DataPipe>>>;

pub trait BufferTrait {
    fn new(fill_size: usize, inputs: &Vec<HalfPipe>, default: f64, name: String) -> BasicBufferProcessor;
    fn get_input_vectors(&self) -> Vec<DataBuffer>;
    fn get_output_vectors(&self) -> Vec<DataBuffer>;
    fn get_fill_length(&self) -> usize;
    fn get_output_halfpipes(&self) -> Vec<HalfPipe>;
    fn get_channels(&self) -> DspChannels;
    fn get_num_channels(&self) -> usize;
    fn create_pipes(&mut self, inputs: &Vec<HalfPipe>) -> ();
    fn populate_data(&mut self, val: f64) -> ();
    fn write_to_file(&self) -> ();
}

// basic struct that will house common components
#[allow(dead_code)]
pub struct BasicBufferProcessor {
    channels: DspChannels,
    vector_fill_length: usize,
    name: String
}

impl BufferTrait for BasicBufferProcessor {
    fn get_input_vectors(&self) -> Vec<DataBuffer> {
        let mut inputs: Vec<DataBuffer> = Vec::new();

        let b = self.channels.lock().unwrap();
        for i in 0..b.len() {
            inputs.push(b[i].get_input_data_vec());
        }
        inputs
    }

    fn get_output_vectors(&self) -> Vec<DataBuffer> {
        let mut outputs: Vec<DataBuffer> = Vec::new();

        let b = self.channels.lock().unwrap();
        for i in 0..b.len() {
            outputs.push(b[i].get_output_data_vec());
        }
        outputs
    }

    fn get_fill_length(&self) -> usize {
        self.vector_fill_length
    }

    fn get_output_halfpipes(&self) -> Vec<HalfPipe> {
        let mut outputs: Vec<HalfPipe> = Vec::new();

        let b = self.channels.lock().unwrap();
        for i in 0..b.len() {
            outputs.push(b[i].get_output_halfpipe());
        }
        outputs
    }

    fn get_channels(&self) -> DspChannels {
        Arc::clone(&self.channels)
    }

    fn get_num_channels(&self) -> usize {
        self.channels.lock().unwrap().len()
    }

    fn new(fill_size: usize, inputs: &Vec<HalfPipe>, default: f64, name: String) -> BasicBufferProcessor {
        let mut bb = BasicBufferProcessor {
            channels: Arc::new(Mutex::new(Vec::new())),
            vector_fill_length: fill_size,
            name: name
        };
        bb.create_pipes(inputs);
        bb.populate_data(default);
        bb
    }
    
    fn create_pipes(&mut self, inputs: &Vec<HalfPipe>) -> () {
        let mut channels = self.channels.lock().unwrap();
        for i in 0..inputs.len() {

            (*channels).push(DataPipe::new(
                Arc::clone(&inputs[i].data), 
                Arc::clone(&inputs[i].data_ready), 
                Arc::clone(&inputs[i].cond_var), 
                i)
            );
        }
    }

    fn populate_data(&mut self, value: f64) -> () {
        let channels = self.channels.lock().unwrap();
        for i in 0..channels.len() {
            let pipe = channels.get(i);
            match pipe {
                None => panic!("Could not locate output vector!"),
                Some(data_pipe) => {
                    for _ in 0..self.vector_fill_length {
                        let val: Complex<f64> = Complex::new(value, value);
                        data_pipe.output.data.lock().unwrap().push(val);
                    }
                }
            };
        }
    }

    fn write_to_file(&self) -> () {
        let binding: DspChannels = self.get_channels();
        let data_pipes = binding.lock().unwrap();

        let filename: String = format!("{}.csv", self.name);
        let mut f = File::create(&filename).expect(&format!("Unable to create file {}!", filename));

        for i in 0..self.get_fill_length() {            
            let mut row_str: String = String::new();

            for j in 0..data_pipes.len() {

                let data_pipe = (*data_pipes).get(j).unwrap();
                let data_vec = data_pipe.get_output_data_vec();

                row_str += &(*data_vec.lock().unwrap())[i].re.to_string();
                row_str += ",";
            }

            row_str += "\n";
            write!(f, "{}", row_str).expect(&format!("Could not write to file {}!", filename));
        }
    }
}

pub trait DspPathMember {
    fn run(&mut self, clk: Arc<Mutex<i8>>, period: i64) -> ();
    fn get_output_halfpipes(&self) -> Vec<HalfPipe>;
}