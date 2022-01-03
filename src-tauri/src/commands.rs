use libcaliph::routines::{self};

#[tauri::command]
pub fn convert(ph: f64, slope: f64, offset: f64) -> f64 {
  routines::ph_convert(&ph, &[slope, offset])
}

#[tauri::command]
pub fn calibrate(ph4: f64, ph10: f64, temperature: f64) -> [f64; 2] {
  let calib = routines::ph_calibration(&[ph4, ph10], &temperature);
  [calib.slope, calib.offset]
}
// #[cfg(test)]
// mod tests {
//   use super::*;
//
//   #[test]
//   fn test_custom() {
//     let result = my_custom_command("test");
//     assert_eq!(
//       result,
//       "I was invoked from JS, with this message: test".to_owned()
//     );
//   }
// }
