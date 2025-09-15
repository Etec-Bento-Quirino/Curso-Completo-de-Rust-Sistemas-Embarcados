# 🍓 Módulo 5: Raspberry Pi e Linux Embarcado

## 🎯 **Objetivos de Aprendizagem Acadêmica**

Ao final deste módulo, você será capaz de:
- ✅ Desenvolver aplicações Rust para Raspberry Pi
- ✅ Trabalhar com GPIO e periféricos
- ✅ Implementar interfaces de câmera e áudio
- ✅ Criar serviços de rede
- ✅ Gerenciar recursos do sistema
- ✅ Desenvolver aplicações Linux embarcadas

## 📚 **Conteúdo Teórico Acadêmico**

### **5.1 Raspberry Pi com Rust**

#### **Configuração do Ambiente**
```bash
# Instalar Rust no Raspberry Pi
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalar dependências do sistema
sudo apt-get update
sudo apt-get install build-essential libssl-dev pkg-config
```

#### **GPIO Control**
```rust
// gpio_control.rs
use rppal::gpio::{Gpio, InputPin, OutputPin, Level};

pub struct RaspberryPiController {
    led_pin: OutputPin,
    button_pin: InputPin,
    gpio: Gpio,
}

impl RaspberryPiController {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let gpio = Gpio::new()?;
        
        let led_pin = gpio.get(18)?.into_output();
        let button_pin = gpio.get(24)?.into_input();
        
        Ok(Self {
            led_pin,
            button_pin,
            gpio,
        })
    }
    
    pub fn control_led(&mut self, state: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.led_pin.write(if state { Level::High } else { Level::Low });
        Ok(())
    }
    
    pub fn is_button_pressed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.button_pin.read() == Level::Low)
    }
}
```

### **5.2 Camera Interface**

```rust
// camera_interface.rs
use std::process::Command;

pub struct CameraController {
    device_path: String,
}

impl CameraController {
    pub fn new(device_path: &str) -> Self {
        Self {
            device_path: device_path.to_string(),
        }
    }
    
    pub fn capture_image(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("raspistill")
            .arg("-o")
            .arg(output_path)
            .arg("-w")
            .arg("1920")
            .arg("-h")
            .arg("1080")
            .output()?;
            
        if !output.status.success() {
            return Err("Failed to capture image".into());
        }
        
        Ok(())
    }
    
    pub fn start_video_stream(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        Command::new("raspivid")
            .arg("-t")
            .arg("0")
            .arg("-w")
            .arg("1280")
            .arg("-h")
            .arg("720")
            .arg("-fps")
            .arg("30")
            .arg("-o")
            .arg(format!("tcp://0.0.0.0:{}", port))
            .spawn()?;
            
        Ok(())
    }
}
```

## 💻 **Exemplos Práticos Acadêmicos**

### **Exemplo 1: Sistema de Segurança Inteligente**

**Objetivo de Pesquisa**: Sistema de monitoramento com detecção de movimento

```rust
// sistema_seguranca.rs
use rppal::gpio::{Gpio, InputPin, OutputPin, Level};
use std::thread;
use std::time::Duration;

pub struct SistemaSeguranca {
    sensor_movimento: InputPin,
    buzzer: OutputPin,
    led_status: OutputPin,
    camera: CameraController,
    ativo: bool,
}

impl SistemaSeguranca {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let gpio = Gpio::new()?;
        
        Ok(Self {
            sensor_movimento: gpio.get(17)?.into_input(),
            buzzer: gpio.get(22)?.into_output(),
            led_status: gpio.get(27)?.into_output(),
            camera: CameraController::new("/dev/video0"),
            ativo: false,
        })
    }
    
    pub fn ativar_sistema(&mut self) {
        self.ativo = true;
        self.led_status.write(Level::High);
    }
    
    pub fn desativar_sistema(&mut self) {
        self.ativo = false;
        self.led_status.write(Level::Low);
    }
    
    pub fn monitorar(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if self.ativo && self.sensor_movimento.read() == Level::High {
                // Movimento detectado
                self.ativar_alerta()?;
            }
            
            thread::sleep(Duration::from_millis(100));
        }
    }
    
    fn ativar_alerta(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Ativar buzzer
        self.buzzer.write(Level::High);
        
        // Capturar imagem
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
            
        let image_path = format!("/tmp/alerta_{}.jpg", timestamp);
        self.camera.capture_image(&image_path)?;
        
        // Desativar buzzer após 5 segundos
        thread::sleep(Duration::from_secs(5));
        self.buzzer.write(Level::Low);
        
        Ok(())
    }
}
```

---

**Próximo Módulo**: [Módulo 6: Sistemas Automotivos](../modulo-06-sistemas-automotivos/README.md)

---

**Desenvolvido com ❤️ para a comunidade acadêmica brasileira**

*ETEC Bento Quirino - Curso Completo de Rust para Sistemas Embarcados*
