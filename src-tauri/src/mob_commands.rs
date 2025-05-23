mod mob_commands;

#[derive(Default)]
struct MyState {
  s: std::sync::Mutex<Vec<String>>,
//   t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn add_mobber(state: tauri::State<'_, MyState>, name: &str) -> Result<(), String> {
  state.s.lock().unwrap().push(name);
  println!(state.s);
//   state.t.lock().unwrap().insert(name.into(), name.into());
  Ok(())
}