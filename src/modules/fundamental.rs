pub fn option_str2float(number_str_option: Option<&&str>) -> f64 {
    number_str_option.unwrap().parse()
}
pub fn option_str2int32(number_str_option: Option<&&str>) -> i32 {
    number_str_option.unwrap().parse()
}
