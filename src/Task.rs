
pub struct Task {
  id : u64,
  titulo: String,
  descripcion: String,
  fecha_vencimiento: String,
  estado: Estado,
}

#[derive(Debug)]
pub enum Estado {
  Pendiente,
  Completada,
}

impl Task {
  pub fn new(id: u64, titulo: String, descripcion: String, fecha_vencimiento: String) -> Task{
    Task {
      id : id,
      titulo : titulo,
      descripcion : descripcion,
      fecha_vencimiento : fecha_vencimiento,
      estado : Estado::Pendiente,
    }
  }
  pub fn new_default() -> Task{
    Task {
      id : 0,
      titulo : "Tarea".to_string(),
      descripcion : "Descripción".to_string(),
      fecha_vencimiento : "2020.06.20".to_string(),
      estado : Estado::Pendiente,
    }
  }
  pub fn get_titulo(self) -> String{ self.titulo }
  pub fn set_titulo(&mut self, newtitulo: String) {self.titulo=newtitulo;}
  pub fn completar(&mut self) {self.estado = Estado::Completada}

  pub fn to_string(&self) -> String {
    format!(
        "ID: {}\nTítulo: {}\nDescripción: {}\nFecha Vencimiento: {}\nEstado: {:?}",
        self.id, self.titulo, self.descripcion, self.fecha_vencimiento, self.estado
    )
}
}