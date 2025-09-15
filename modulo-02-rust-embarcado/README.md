# ⚡ Módulo 2: Rust no Contexto Embarcado

## 🎯 **Objetivos de Aprendizagem Acadêmica**

Ao final deste módulo, você será capaz de:
- ✅ Compreender o modelo `no_std` do Rust
- ✅ Implementar memory safety em ambientes restritos
- ✅ Trabalhar com interrupções e concorrência
- ✅ Desenvolver sistemas de gerenciamento de memória
- ✅ Implementar abstrações de hardware seguras
- ✅ Analisar trade-offs entre segurança e performance

## 📋 **Pré-requisitos**

### **Obrigatórios**
- ✅ Conhecimento básico de Rust
- ✅ Conceitos de sistemas embarcados
- ✅ Familiaridade com microcontroladores
- ✅ Conhecimento de programação em C/C++

### **Recomendados**
- ✅ Experiência com `no_std` programming
- ✅ Conhecimento de assembly básico
- ✅ Familiaridade com debugging
- ✅ Experiência com sistemas em tempo real

### **Recursos de Aprendizado**
- 📚 [The Rust Programming Language](https://doc.rust-lang.org/book/)
- 🔧 [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- ⚡ [Embedded Rust Discovery](https://docs.rust-embedded.org/discovery/)

## 📚 **Conteúdo Teórico Acadêmico**

### **📋 Índice do Módulo**
- [2.1 Introdução ao no_std](#21-introdução-ao-no_std)
- [2.2 Memory Safety em Embarcados](#22-memory-safety-em-embarcados)
- [2.3 Sistema de Interrupções](#23-sistema-de-interrupções)
- [2.4 Gerenciamento de Memória](#24-gerenciamento-de-memória)
- [Exemplos Práticos](#exemplos-práticos-acadêmicos)
- [Projeto Acadêmico](#projeto-acadêmico-sistema-crítico-em-tempo-real)
- [Atividades Acadêmicas](#atividades-acadêmicas)

---

### **2.1 Introdução ao no_std**

#### **Conceitos Fundamentais**
```rust
#![no_std]  // Sem biblioteca padrão
#![no_main] // Sem função main padrão
#![feature(start)] // Feature para entry point

// Sem heap allocation
// Sem std::collections
// Sem std::thread
// Sem std::sync
```

#### **Vantagens do no_std**
- **Controle Total**: Acesso direto ao hardware
- **Previsibilidade**: Sem garbage collection
- **Performance**: Sem overhead de runtime
- **Segurança**: Memory safety mantida

### **2.2 Memory Safety em Embarcados**

#### **Ownership em Sistemas Críticos**
```rust
// ownership_embarcado.rs
pub struct CriticalResource {
    data: [u8; 1024],
    is_initialized: bool,
}

impl CriticalResource {
    pub fn new() -> Self {
        Self {
            data: [0; 1024],
            is_initialized: false,
        }
    }
    
    pub fn initialize(&mut self) -> Result<(), CriticalError> {
        if self.is_initialized {
            return Err(CriticalError::AlreadyInitialized);
        }
        
        // Inicialização segura
        for i in 0..self.data.len() {
            self.data[i] = i as u8;
        }
        
        self.is_initialized = true;
        Ok(())
    }
    
    pub fn read_data(&self, index: usize) -> Result<u8, CriticalError> {
        if !self.is_initialized {
            return Err(CriticalError::NotInitialized);
        }
        
        if index >= self.data.len() {
            return Err(CriticalError::IndexOutOfBounds);
        }
        
        Ok(self.data[index])
    }
    
    pub fn write_data(&mut self, index: usize, value: u8) -> Result<(), CriticalError> {
        if !self.is_initialized {
            return Err(CriticalError::NotInitialized);
        }
        
        if index >= self.data.len() {
            return Err(CriticalError::IndexOutOfBounds);
        }
        
        self.data[index] = value;
        Ok(())
    }
}

#[derive(Debug)]
pub enum CriticalError {
    NotInitialized,
    AlreadyInitialized,
    IndexOutOfBounds,
    MemoryError,
}
```

#### **Borrowing em Contextos de Interrupção**
```rust
// borrowing_interrupts.rs
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use core::cell::UnsafeCell;

// Dados compartilhados entre interrupção e main loop
pub struct SharedData {
    counter: AtomicU32,
    flag: AtomicBool,
    buffer: UnsafeCell<[u8; 256]>,
}

unsafe impl Sync for SharedData {}

impl SharedData {
    pub fn new() -> Self {
        Self {
            counter: AtomicU32::new(0),
            flag: AtomicBool::new(false),
            buffer: UnsafeCell::new([0; 256]),
        }
    }
    
    pub fn increment_counter(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn set_flag(&self, value: bool) {
        self.flag.store(value, Ordering::Relaxed);
    }
    
    pub fn is_flag_set(&self) -> bool {
        self.flag.load(Ordering::Relaxed)
    }
    
    pub fn get_counter(&self) -> u32 {
        self.counter.load(Ordering::Relaxed)
    }
    
    pub fn write_buffer(&self, index: usize, value: u8) -> Result<(), CriticalError> {
        if index >= 256 {
            return Err(CriticalError::IndexOutOfBounds);
        }
        
        unsafe {
            (*self.buffer.get())[index] = value;
        }
        
        Ok(())
    }
    
    pub fn read_buffer(&self, index: usize) -> Result<u8, CriticalError> {
        if index >= 256 {
            return Err(CriticalError::IndexOutOfBounds);
        }
        
        Ok(unsafe { (*self.buffer.get())[index] })
    }
}
```

### **2.3 Sistema de Interrupções**

#### **Gerenciador de Interrupções**
```rust
// interrupt_manager.rs
use core::sync::atomic::{AtomicU32, Ordering};

pub struct InterruptManager {
    interrupt_counters: [AtomicU32; 8],
    interrupt_enabled: [AtomicBool; 8],
}

impl InterruptManager {
    pub fn new() -> Self {
        Self {
            interrupt_counters: [
                AtomicU32::new(0), AtomicU32::new(0),
                AtomicU32::new(0), AtomicU32::new(0),
                AtomicU32::new(0), AtomicU32::new(0),
                AtomicU32::new(0), AtomicU32::new(0),
            ],
            interrupt_enabled: [
                AtomicBool::new(false), AtomicBool::new(false),
                AtomicBool::new(false), AtomicBool::new(false),
                AtomicBool::new(false), AtomicBool::new(false),
                AtomicBool::new(false), AtomicBool::new(false),
            ],
        }
    }
    
    pub fn enable_interrupt(&self, interrupt_id: usize) {
        if interrupt_id < 8 {
            self.interrupt_enabled[interrupt_id].store(true, Ordering::Relaxed);
        }
    }
    
    pub fn disable_interrupt(&self, interrupt_id: usize) {
        if interrupt_id < 8 {
            self.interrupt_enabled[interrupt_id].store(false, Ordering::Relaxed);
        }
    }
    
    pub fn handle_interrupt(&self, interrupt_id: usize) {
        if interrupt_id < 8 && self.interrupt_enabled[interrupt_id].load(Ordering::Relaxed) {
            self.interrupt_counters[interrupt_id].fetch_add(1, Ordering::Relaxed);
        }
    }
    
    pub fn get_interrupt_count(&self, interrupt_id: usize) -> u32 {
        if interrupt_id < 8 {
            self.interrupt_counters[interrupt_id].load(Ordering::Relaxed)
        } else {
            0
        }
    }
}

// Macros para definir handlers de interrupção
#[macro_export]
macro_rules! interrupt_handler {
    ($name:ident, $id:expr) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            INTERRUPT_MANAGER.handle_interrupt($id);
        }
    };
}

// Exemplo de uso
interrupt_handler!(timer0_interrupt, 0);
interrupt_handler!(uart_interrupt, 1);
interrupt_handler!(adc_interrupt, 2);
```

### **2.4 Gerenciamento de Memória**

#### **Allocator Personalizado**
```rust
// custom_allocator.rs
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

pub struct EmbeddedAllocator {
    heap_start: *mut u8,
    heap_end: *mut u8,
    current_ptr: *mut u8,
}

unsafe impl Sync for EmbeddedAllocator {}

impl EmbeddedAllocator {
    pub const fn new(heap_start: *mut u8, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_end: unsafe { heap_start.add(heap_size) },
            current_ptr: heap_start,
        }
    }
    
    pub fn used_memory(&self) -> usize {
        unsafe { self.current_ptr.offset_from(self.heap_start) as usize }
    }
    
    pub fn available_memory(&self) -> usize {
        unsafe { self.heap_end.offset_from(self.current_ptr) as usize }
    }
}

unsafe impl GlobalAlloc for EmbeddedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        
        // Alinhar ponteiro
        let aligned_ptr = (self.current_ptr as usize + align - 1) & !(align - 1);
        let aligned_ptr = aligned_ptr as *mut u8;
        
        // Verificar se há memória suficiente
        if aligned_ptr.add(size) > self.heap_end {
            return core::ptr::null_mut();
        }
        
        // Atualizar ponteiro atual
        self.current_ptr = aligned_ptr.add(size);
        
        aligned_ptr
    }
    
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Em sistemas embarcados simples, não fazemos deallocação
        // A memória é liberada apenas no reset
    }
}

// Uso do allocator
#[global_allocator]
static ALLOCATOR: EmbeddedAllocator = EmbeddedAllocator::new(
    unsafe { core::ptr::null_mut() }, // Será configurado no startup
    8192 // 8KB de heap
);
```

## 💻 **Exemplos Práticos Acadêmicos**

### **Exemplo 1: Sistema de Gerenciamento de Recursos**

**Objetivo de Pesquisa**: Implementar sistema seguro de gerenciamento de recursos

```rust
// resource_manager.rs
pub struct ResourceManager {
    resources: [Option<Resource>; 16],
    allocation_map: [bool; 16],
}

#[derive(Debug, Clone)]
pub struct Resource {
    id: u8,
    priority: u8,
    memory_size: usize,
    is_critical: bool,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: [None; 16],
            allocation_map: [false; 16],
        }
    }
    
    pub fn allocate_resource(&mut self, resource: Resource) -> Result<u8, ResourceError> {
        // Encontrar slot livre
        for i in 0..16 {
            if !self.allocation_map[i] {
                self.resources[i] = Some(resource);
                self.allocation_map[i] = true;
                return Ok(i as u8);
            }
        }
        
        Err(ResourceError::NoAvailableSlots)
    }
    
    pub fn deallocate_resource(&mut self, id: u8) -> Result<(), ResourceError> {
        if id >= 16 {
            return Err(ResourceError::InvalidId);
        }
        
        if !self.allocation_map[id as usize] {
            return Err(ResourceError::NotAllocated);
        }
        
        self.resources[id as usize] = None;
        self.allocation_map[id as usize] = false;
        
        Ok(())
    }
    
    pub fn get_resource(&self, id: u8) -> Result<&Resource, ResourceError> {
        if id >= 16 {
            return Err(ResourceError::InvalidId);
        }
        
        self.resources[id as usize].as_ref()
            .ok_or(ResourceError::NotAllocated)
    }
    
    pub fn get_resource_usage(&self) -> ResourceUsage {
        let allocated = self.allocation_map.iter().filter(|&&x| x).count();
        let total_memory: usize = self.resources.iter()
            .filter_map(|r| r.as_ref())
            .map(|r| r.memory_size)
            .sum();
        
        ResourceUsage {
            allocated_resources: allocated,
            total_memory_used: total_memory,
            available_slots: 16 - allocated,
        }
    }
}

#[derive(Debug)]
pub enum ResourceError {
    NoAvailableSlots,
    InvalidId,
    NotAllocated,
    InsufficientMemory,
}

#[derive(Debug)]
pub struct ResourceUsage {
    pub allocated_resources: usize,
    pub total_memory_used: usize,
    pub available_slots: usize,
}
```

### **Exemplo 2: Sistema de Scheduler em Tempo Real**

**Objetivo de Pesquisa**: Implementar scheduler determinístico para sistemas críticos

```rust
// real_time_scheduler.rs
use core::sync::atomic::{AtomicU32, Ordering};

pub struct RealTimeScheduler {
    tasks: [Option<Task>; 8],
    current_task: AtomicU32,
    system_tick: AtomicU32,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u8,
    pub priority: u8,
    pub period: u32,
    pub deadline: u32,
    pub execution_time: u32,
    pub last_execution: u32,
    pub is_periodic: bool,
}

impl RealTimeScheduler {
    pub fn new() -> Self {
        Self {
            tasks: [None; 8],
            current_task: AtomicU32::new(0),
            system_tick: AtomicU32::new(0),
        }
    }
    
    pub fn add_task(&mut self, task: Task) -> Result<u8, SchedulerError> {
        // Encontrar slot livre
        for i in 0..8 {
            if self.tasks[i].is_none() {
                self.tasks[i] = Some(task);
                return Ok(i as u8);
            }
        }
        
        Err(SchedulerError::NoAvailableSlots)
    }
    
    pub fn remove_task(&mut self, task_id: u8) -> Result<(), SchedulerError> {
        if task_id >= 8 {
            return Err(SchedulerError::InvalidTaskId);
        }
        
        if self.tasks[task_id as usize].is_none() {
            return Err(SchedulerError::TaskNotFound);
        }
        
        self.tasks[task_id as usize] = None;
        Ok(())
    }
    
    pub fn schedule(&mut self) -> Option<u8> {
        let current_time = self.system_tick.load(Ordering::Relaxed);
        let mut highest_priority = 0;
        let mut selected_task = None;
        
        // Rate Monotonic Scheduling
        for (i, task_opt) in self.tasks.iter().enumerate() {
            if let Some(task) = task_opt {
                if task.is_periodic && (current_time - task.last_execution) >= task.period {
                    if task.priority > highest_priority {
                        highest_priority = task.priority;
                        selected_task = Some(i as u8);
                    }
                }
            }
        }
        
        if let Some(task_id) = selected_task {
            if let Some(task) = &mut self.tasks[task_id as usize] {
                task.last_execution = current_time;
            }
        }
        
        selected_task
    }
    
    pub fn tick(&self) {
        self.system_tick.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn check_deadline_miss(&self, task_id: u8) -> bool {
        if task_id >= 8 {
            return false;
        }
        
        if let Some(task) = &self.tasks[task_id as usize] {
            let current_time = self.system_tick.load(Ordering::Relaxed);
            let elapsed = current_time - task.last_execution;
            return elapsed > task.deadline;
        }
        
        false
    }
    
    pub fn get_system_load(&self) -> f32 {
        let total_utilization: f32 = self.tasks.iter()
            .filter_map(|t| t.as_ref())
            .map(|task| task.execution_time as f32 / task.period as f32)
            .sum();
        
        total_utilization * 100.0
    }
}

#[derive(Debug)]
pub enum SchedulerError {
    NoAvailableSlots,
    InvalidTaskId,
    TaskNotFound,
    DeadlineMiss,
    SystemOverload,
}
```

## 🛠️ **Projeto Acadêmico: Sistema Crítico em Tempo Real**

### **Objetivo**
Desenvolver um sistema crítico em tempo real com garantias de segurança e determinismo.

### **Funcionalidades**
- ✅ Gerenciamento seguro de recursos
- ✅ Scheduler determinístico
- ✅ Sistema de interrupções robusto
- ✅ Memory safety garantida
- ✅ Análise de performance em tempo real

### **Estrutura do Projeto**
```
sistema-critico/
├── src/
│   ├── main.rs
│   ├── scheduler/
│   │   ├── mod.rs
│   │   ├── real_time.rs
│   │   └── priority.rs
│   ├── memory/
│   │   ├── mod.rs
│   │   ├── allocator.rs
│   │   └── manager.rs
│   ├── interrupts/
│   │   ├── mod.rs
│   │   ├── handler.rs
│   │   └── manager.rs
│   └── safety/
│       ├── mod.rs
│       ├── watchdog.rs
│       └── checks.rs
├── Cargo.toml
└── README.md
```

## 🎯 **Atividades Acadêmicas**

### **Atividade 1: Análise de Memory Safety**
- Implementar sistema de detecção de vazamentos
- Comparar com sistemas em C
- Medir overhead de segurança

### **Atividade 2: Otimização de Performance**
- Implementar profiling em tempo real
- Analisar latência de interrupções
- Otimizar scheduler para determinismo

### **Atividade 3: Validação Formal**
- Implementar verificações estáticas
- Usar ferramentas de análise formal
- Documentar garantias de segurança

## 📊 **Métricas de Avaliação**

### **Implementação (50%)**
- Correção da implementação
- Segurança de memória
- Performance e determinismo
- Tratamento de erros

### **Pesquisa (30%)**
- Análise comparativa
- Metodologia científica
- Documentação técnica
- Conclusões fundamentadas

### **Apresentação (20%)**
- Demonstração prática
- Clareza na exposição
- Resposta a questionamentos
- Qualidade do relatório

---

## 🧭 **Navegação**

### **📚 Material de Apoio**
- [**README Principal**](../../README.md) - Visão geral do curso
- [**Tutoriais Detalhados**](../../TUTORIAIS.md) - Guia completo de tutoriais
- [**Módulo 1: Fundamentos**](../modulo-01-fundamentos-embarcados/README.md) - Módulo anterior
- [**Módulo 3: Arduino**](../modulo-03-arduino-rust/README.md) - Próximo módulo

### **🔗 Links Úteis**
- [Rust Embedded Working Group](https://github.com/rust-embedded/wg)
- [Arduino Rust Community](https://github.com/Rahix/avr-hal)
- [ESP32 Rust Community](https://github.com/esp-rs)

### **📖 Documentação Oficial**
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- [Embedded Rust Discovery](https://docs.rust-embedded.org/discovery/)

---

**Próximo Módulo**: [Módulo 3: Arduino com Rust](../modulo-03-arduino-rust/README.md)

---

**Desenvolvido com ❤️ para a comunidade acadêmica brasileira**

*ETEC Bento Quirino - Curso Acadêmico de Rust para Sistemas Embarcados*
