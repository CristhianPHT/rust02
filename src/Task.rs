struct Task {
  id : u64,
  titulo: String,
  descripcion: String,
  fecha_vencimiento: String,
  estado: Estado,
}

enum Estado {
  Pendiente,
  Completada,
}

impl Task {
  fn Task() -> Task{
    Task {
      id : 0,
      titulo : "Tarea".to_string(),
      descripcion : "Descripción".to_string(),
      fecha_vencimiento : "2020.06.20".to_string(),
      estado : Estado::Pendiente,
    }
  }
  fn getTitulo(&self) -> &str{
    &self.titulo
  }
}