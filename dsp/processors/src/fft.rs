
use bb_processor::BasicBufferProcessor;

pub struct FFT {
    base_processor: BasicBufferProcessor,
    // todo?
}


impl FFT {
    pub fn new(){
        base_processor: BasicBufferProcessor::new();

        // todo: anything else here?
    }

    pub fn run() -> () {
        base_processor.wait_for_start();

        // spawn thread for each channel and perform FFT

        // join all threads

        base_processor.end_of_processing();
    }

    fn performFFT(){
        // todo port this basic fft algo

        // for (int i = 0; i < BUFFER_LEN; ++i){
        //     for(int j = 0; j < BUFFER_LEN; ++j){
        //         // i is for each f_i element
        //         // j is for going through each array element and weighting
        //         T trig_arg = (2.0 * PI * i * j)/BUFFER_LEN;
        //         T real_product = data_buffer[channel][j] * cos(trig_arg);
        //         T imag_product = data_buffer[channel][j] * sin(trig_arg);
        //         this->real_buffer[channel][i] += real_product;
        //         this->imag_buffer[channel][i] -= imag_product;
        //     }
        // }
    }
}