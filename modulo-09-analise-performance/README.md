# 📊 Módulo 9: Análise de Performance

## 🎯 **Objetivos de Aprendizagem Acadêmica**

Ao final deste módulo, você será capaz de:
- ✅ Implementar profiling e benchmarking em sistemas embarcados
- ✅ Analisar consumo energético e power management
- ✅ Medir métricas de performance em tempo real
- ✅ Otimizar código para sistemas com recursos limitados
- ✅ Desenvolver ferramentas de análise de performance
- ✅ Criar relatórios de performance para pesquisa acadêmica

## 📋 **Pré-requisitos**

### **Obrigatórios**
- ✅ Conhecimento básico de Rust
- ✅ Conceitos de sistemas embarcados
- ✅ Familiaridade com profiling
- ✅ Conhecimento de análise de performance

### **Recomendados**
- ✅ Experiência com benchmarking
- ✅ Conhecimento de power management
- ✅ Familiaridade com ferramentas de análise
- ✅ Experiência com projetos de hardware

### **Recursos de Aprendizado**
- 📚 [The Rust Programming Language](https://doc.rust-lang.org/book/)
- 🔧 [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- ⚡ [Embedded Rust Discovery](https://docs.rust-embedded.org/discovery/)
- 📊 [Rust Performance Documentation](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

## 📚 **Conteúdo Teórico Acadêmico**

### **📋 Índice do Módulo**
- [9.1 Profiling e Benchmarking](#91-profiling-e-benchmarking)
- [9.2 Análise de Consumo Energético](#92-análise-de-consumo-energético)
- [9.3 Otimização de Performance](#93-otimização-de-performance)
- [Exemplos Práticos](#exemplos-práticos-acadêmicos)
- [Projeto Acadêmico](#projeto-acadêmico-análise-comparativa-de-eficiência)
- [Atividades Acadêmicas](#atividades-acadêmicas)

---

### **9.1 Profiling e Benchmarking**

#### **Sistema de Profiling**
```rust
// performance_profiler.rs
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use core::cell::UnsafeCell;

pub struct PerformanceProfiler {
    execution_times: UnsafeCell<[u32; 100]>,
    memory_usage: UnsafeCell<[usize; 100]>,
    power_consumption: UnsafeCell<[f32; 100]>,
    sample_count: AtomicU32,
    start_time: AtomicU64,
}

unsafe impl Sync for PerformanceProfiler {}

impl PerformanceProfiler {
    pub const fn new() -> Self {
        Self {
            execution_times: UnsafeCell::new([0; 100]),
            memory_usage: UnsafeCell::new([0; 100]),
            power_consumption: UnsafeCell::new([0.0; 100]),
            sample_count: AtomicU32::new(0),
            start_time: AtomicU64::new(0),
        }
    }
    
    pub fn start_profiling(&self) {
        self.start_time.store(self.get_system_time(), Ordering::Relaxed);
    }
    
    pub fn stop_profiling(&self) -> ProfilingResult {
        let end_time = self.get_system_time();
        let execution_time = end_time - self.start_time.load(Ordering::Relaxed);
        
        let sample_index = self.sample_count.fetch_add(1, Ordering::Relaxed) % 100;
        
        unsafe {
            (*self.execution_times.get())[sample_index as usize] = execution_time as u32;
            (*self.memory_usage.get())[sample_index as usize] = self.get_current_memory_usage();
            (*self.power_consumption.get())[sample_index as usize] = self.get_current_power_consumption();
        }
        
        ProfilingResult {
            execution_time,
            memory_usage: self.get_current_memory_usage(),
            power_consumption: self.get_current_power_consumption(),
            cpu_usage: self.calculate_cpu_usage(),
        }
    }
    
    pub fn get_statistics(&self) -> PerformanceStatistics {
        let sample_count = self.sample_count.load(Ordering::Relaxed) as usize;
        let actual_samples = sample_count.min(100);
        
        if actual_samples == 0 {
            return PerformanceStatistics::default();
        }
        
        unsafe {
            let execution_times = &(*self.execution_times.get())[..actual_samples];
            let memory_usage = &(*self.memory_usage.get())[..actual_samples];
            let power_consumption = &(*self.power_consumption.get())[..actual_samples];
            
            PerformanceStatistics {
                avg_execution_time: execution_times.iter().sum::<u32>() as f32 / actual_samples as f32,
                min_execution_time: *execution_times.iter().min().unwrap_or(&0) as f32,
                max_execution_time: *execution_times.iter().max().unwrap_or(&0) as f32,
                avg_memory_usage: memory_usage.iter().sum::<usize>() as f32 / actual_samples as f32,
                max_memory_usage: *memory_usage.iter().max().unwrap_or(&0),
                avg_power_consumption: power_consumption.iter().sum::<f32>() / actual_samples as f32,
                min_power_consumption: power_consumption.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
                max_power_consumption: power_consumption.iter().fold(0.0, |a, &b| a.max(b)),
                sample_count: actual_samples,
            }
        }
    }
    
    fn get_system_time(&self) -> u64 {
        // Implementar com timer do sistema
        0
    }
    
    fn get_current_memory_usage(&self) -> usize {
        // Implementar medição de memória
        0
    }
    
    fn get_current_power_consumption(&self) -> f32 {
        // Implementar medição de energia
        0.0
    }
    
    fn calculate_cpu_usage(&self) -> f32 {
        // Implementar cálculo de uso de CPU
        0.0
    }
}

#[derive(Debug, Clone)]
pub struct ProfilingResult {
    pub execution_time: u64,
    pub memory_usage: usize,
    pub power_consumption: f32,
    pub cpu_usage: f32,
}

#[derive(Debug, Clone)]
pub struct PerformanceStatistics {
    pub avg_execution_time: f32,
    pub min_execution_time: f32,
    pub max_execution_time: f32,
    pub avg_memory_usage: f32,
    pub max_memory_usage: usize,
    pub avg_power_consumption: f32,
    pub min_power_consumption: f32,
    pub max_power_consumption: f32,
    pub sample_count: usize,
}

impl Default for PerformanceStatistics {
    fn default() -> Self {
        Self {
            avg_execution_time: 0.0,
            min_execution_time: 0.0,
            max_execution_time: 0.0,
            avg_memory_usage: 0.0,
            max_memory_usage: 0,
            avg_power_consumption: 0.0,
            min_power_consumption: 0.0,
            max_power_consumption: 0.0,
            sample_count: 0,
        }
    }
}

static PROFILER: PerformanceProfiler = PerformanceProfiler::new();

// Macro para profiling automático
#[macro_export]
macro_rules! profile_function {
    ($name:expr, $code:block) => {
        PROFILER.start_profiling();
        let result = $code;
        let profiling_result = PROFILER.stop_profiling();
        log_info!("Function {}: {}μs, {}B, {}mW", 
            $name, 
            profiling_result.execution_time, 
            profiling_result.memory_usage,
            profiling_result.power_consumption
        );
        result
    };
}
```

### **9.2 Power Management**

```rust
// power_manager.rs
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

pub struct PowerManager {
    current_mode: AtomicU32,
    power_consumption: AtomicU32,
    battery_level: AtomicU32,
    sleep_enabled: AtomicBool,
}

#[derive(Debug, Clone, Copy)]
pub enum PowerMode {
    Active = 0,
    Sleep = 1,
    DeepSleep = 2,
    Hibernate = 3,
}

impl PowerManager {
    pub const fn new() -> Self {
        Self {
            current_mode: AtomicU32::new(PowerMode::Active as u32),
            power_consumption: AtomicU32::new(100), // mW
            battery_level: AtomicU32::new(100), // %
            sleep_enabled: AtomicBool::new(true),
        }
    }
    
    pub fn set_power_mode(&self, mode: PowerMode) {
        self.current_mode.store(mode as u32, Ordering::Relaxed);
        
        match mode {
            PowerMode::Active => {
                self.power_consumption.store(100, Ordering::Relaxed);
                self.enable_all_peripherals();
            }
            PowerMode::Sleep => {
                self.power_consumption.store(10, Ordering::Relaxed);
                self.disable_unused_peripherals();
            }
            PowerMode::DeepSleep => {
                self.power_consumption.store(1, Ordering::Relaxed);
                self.disable_most_peripherals();
            }
            PowerMode::Hibernate => {
                self.power_consumption.store(0, Ordering::Relaxed);
                self.disable_all_peripherals();
            }
        }
    }
    
    pub fn get_power_consumption(&self) -> u32 {
        self.power_consumption.load(Ordering::Relaxed)
    }
    
    pub fn get_battery_level(&self) -> u32 {
        self.battery_level.load(Ordering::Relaxed)
    }
    
    pub fn update_battery_level(&self, level: u32) {
        self.battery_level.store(level, Ordering::Relaxed);
        
        // Ajustar modo de energia baseado no nível da bateria
        if level < 20 {
            self.set_power_mode(PowerMode::Hibernate);
        } else if level < 50 {
            self.set_power_mode(PowerMode::DeepSleep);
        } else if level < 80 {
            self.set_power_mode(PowerMode::Sleep);
        } else {
            self.set_power_mode(PowerMode::Active);
        }
    }
    
    pub fn optimize_power(&self) -> PowerOptimizationResult {
        let current_consumption = self.get_power_consumption();
        let battery_level = self.get_battery_level();
        
        let optimized_mode = if battery_level < 20 {
            PowerMode::Hibernate
        } else if battery_level < 50 {
            PowerMode::DeepSleep
        } else if battery_level < 80 {
            PowerMode::Sleep
        } else {
            PowerMode::Active
        };
        
        self.set_power_mode(optimized_mode);
        
        PowerOptimizationResult {
            previous_mode: PowerMode::Active, // Implementar histórico
            optimized_mode,
            power_saved: current_consumption.saturating_sub(self.get_power_consumption()),
            estimated_battery_life: self.calculate_battery_life(),
        }
    }
    
    fn enable_all_peripherals(&self) {
        // Implementar habilitação de periféricos
    }
    
    fn disable_unused_peripherals(&self) {
        // Implementar desabilitação de periféricos não utilizados
    }
    
    fn disable_most_peripherals(&self) {
        // Implementar desabilitação da maioria dos periféricos
    }
    
    fn disable_all_peripherals(&self) {
        // Implementar desabilitação de todos os periféricos
    }
    
    fn calculate_battery_life(&self) -> u32 {
        let consumption = self.get_power_consumption();
        let battery_level = self.get_battery_level();
        
        if consumption == 0 {
            return u32::MAX; // Bateria infinita em hibernação
        }
        
        // Cálculo simplificado: (nível_bateria * capacidade_bateria) / consumo
        (battery_level * 1000) / consumption // horas estimadas
    }
}

#[derive(Debug)]
pub struct PowerOptimizationResult {
    pub previous_mode: PowerMode,
    pub optimized_mode: PowerMode,
    pub power_saved: u32,
    pub estimated_battery_life: u32,
}

static POWER_MANAGER: PowerManager = PowerManager::new();
```

### **9.3 Real-time Performance Metrics**

```rust
// real_time_metrics.rs
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub struct RealTimeMetrics {
    cpu_usage: AtomicU32,
    memory_usage: AtomicU32,
    stack_usage: AtomicU32,
    heap_usage: AtomicU32,
    interrupt_count: AtomicU32,
    task_switches: AtomicU32,
    context_switches: AtomicU32,
    last_update: AtomicU64,
}

impl RealTimeMetrics {
    pub const fn new() -> Self {
        Self {
            cpu_usage: AtomicU32::new(0),
            memory_usage: AtomicU32::new(0),
            stack_usage: AtomicU32::new(0),
            heap_usage: AtomicU32::new(0),
            interrupt_count: AtomicU32::new(0),
            task_switches: AtomicU32::new(0),
            context_switches: AtomicU32::new(0),
            last_update: AtomicU64::new(0),
        }
    }
    
    pub fn update_cpu_usage(&self, usage: u32) {
        self.cpu_usage.store(usage, Ordering::Relaxed);
        self.update_timestamp();
    }
    
    pub fn update_memory_usage(&self, usage: u32) {
        self.memory_usage.store(usage, Ordering::Relaxed);
        self.update_timestamp();
    }
    
    pub fn increment_interrupt_count(&self) {
        self.interrupt_count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn increment_task_switches(&self) {
        self.task_switches.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_metrics(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            cpu_usage: self.cpu_usage.load(Ordering::Relaxed),
            memory_usage: self.memory_usage.load(Ordering::Relaxed),
            stack_usage: self.stack_usage.load(Ordering::Relaxed),
            heap_usage: self.heap_usage.load(Ordering::Relaxed),
            interrupt_count: self.interrupt_count.load(Ordering::Relaxed),
            task_switches: self.task_switches.load(Ordering::Relaxed),
            context_switches: self.context_switches.load(Ordering::Relaxed),
            timestamp: self.last_update.load(Ordering::Relaxed),
        }
    }
    
    pub fn calculate_performance_score(&self) -> f32 {
        let metrics = self.get_metrics();
        
        // Score baseado em múltiplos fatores
        let cpu_score = if metrics.cpu_usage < 50 { 100.0 } else { 100.0 - (metrics.cpu_usage - 50) as f32 };
        let memory_score = if metrics.memory_usage < 80 { 100.0 } else { 100.0 - (metrics.memory_usage - 80) as f32 * 2.0 };
        let interrupt_score = if metrics.interrupt_count < 1000 { 100.0 } else { 100.0 - (metrics.interrupt_count - 1000) as f32 / 100.0 };
        
        (cpu_score + memory_score + interrupt_score) / 3.0
    }
    
    fn update_timestamp(&self) {
        self.last_update.store(self.get_system_time(), Ordering::Relaxed);
    }
    
    fn get_system_time(&self) -> u64 {
        // Implementar com timer do sistema
        0
    }
}

#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub cpu_usage: u32,
    pub memory_usage: u32,
    pub stack_usage: u32,
    pub heap_usage: u32,
    pub interrupt_count: u32,
    pub task_switches: u32,
    pub context_switches: u32,
    pub timestamp: u64,
}

static REAL_TIME_METRICS: RealTimeMetrics = RealTimeMetrics::new();
```

## 💻 **Exemplos Práticos Acadêmicos**

### **Exemplo 1: Análise Comparativa de Performance**

**Objetivo de Pesquisa**: Comparar performance de diferentes algoritmos em sistemas embarcados

```rust
// performance_comparison.rs
use std::collections::HashMap;

pub struct PerformanceComparison {
    algorithms: HashMap<String, Box<dyn BenchmarkableAlgorithm>>,
    results: Vec<AlgorithmResult>,
}

pub trait BenchmarkableAlgorithm {
    fn name(&self) -> &str;
    fn run(&mut self, input: &[u32]) -> Result<Vec<u32>, AlgorithmError>;
    fn get_memory_usage(&self) -> usize;
    fn get_complexity(&self) -> AlgorithmComplexity;
}

#[derive(Debug, Clone)]
pub struct AlgorithmResult {
    pub algorithm_name: String,
    pub execution_time: u64,
    pub memory_usage: usize,
    pub power_consumption: f32,
    pub accuracy: f32,
    pub complexity: AlgorithmComplexity,
}

#[derive(Debug, Clone)]
pub struct AlgorithmComplexity {
    pub time_complexity: String,
    pub space_complexity: String,
}

impl PerformanceComparison {
    pub fn new() -> Self {
        Self {
            algorithms: HashMap::new(),
            results: Vec::new(),
        }
    }
    
    pub fn add_algorithm(&mut self, algorithm: Box<dyn BenchmarkableAlgorithm>) {
        self.algorithms.insert(algorithm.name().to_string(), algorithm);
    }
    
    pub fn run_comparison(&mut self, test_data: &[u32]) -> ComparisonReport {
        let mut results = Vec::new();
        
        for (name, algorithm) in &mut self.algorithms {
            let result = self.benchmark_algorithm(algorithm, test_data);
            results.push(result);
        }
        
        self.results = results.clone();
        
        ComparisonReport {
            results,
            best_performance: self.find_best_algorithm(),
            summary: self.generate_summary(),
        }
    }
    
    fn benchmark_algorithm(&self, algorithm: &mut Box<dyn BenchmarkableAlgorithm>, test_data: &[u32]) -> AlgorithmResult {
        PROFILER.start_profiling();
        
        let start_memory = self.get_current_memory_usage();
        let result = algorithm.run(test_data);
        let end_memory = self.get_current_memory_usage();
        
        let profiling_result = PROFILER.stop_profiling();
        
        AlgorithmResult {
            algorithm_name: algorithm.name().to_string(),
            execution_time: profiling_result.execution_time,
            memory_usage: end_memory.saturating_sub(start_memory),
            power_consumption: profiling_result.power_consumption,
            accuracy: self.calculate_accuracy(&result, test_data),
            complexity: algorithm.get_complexity(),
        }
    }
    
    fn find_best_algorithm(&self) -> String {
        if self.results.is_empty() {
            return "N/A".to_string();
        }
        
        let best = self.results.iter()
            .min_by(|a, b| a.execution_time.cmp(&b.execution_time))
            .unwrap();
        
        best.algorithm_name.clone()
    }
    
    fn generate_summary(&self) -> String {
        if self.results.is_empty() {
            return "No results available".to_string();
        }
        
        let avg_time = self.results.iter().map(|r| r.execution_time).sum::<u64>() as f32 / self.results.len() as f32;
        let avg_memory = self.results.iter().map(|r| r.memory_usage).sum::<usize>() as f32 / self.results.len() as f32;
        
        format!(
            "Average execution time: {:.2}μs, Average memory usage: {:.2}B, Algorithms tested: {}",
            avg_time, avg_memory, self.results.len()
        )
    }
    
    fn calculate_accuracy(&self, result: &Result<Vec<u32>, AlgorithmError>, expected: &[u32]) -> f32 {
        match result {
            Ok(output) => {
                if output.len() != expected.len() {
                    return 0.0;
                }
                
                let correct = output.iter().zip(expected.iter())
                    .filter(|(a, b)| a == b)
                    .count();
                
                correct as f32 / expected.len() as f32 * 100.0
            }
            Err(_) => 0.0,
        }
    }
    
    fn get_current_memory_usage(&self) -> usize {
        // Implementar medição de memória
        0
    }
}

#[derive(Debug)]
pub struct ComparisonReport {
    pub results: Vec<AlgorithmResult>,
    pub best_performance: String,
    pub summary: String,
}

#[derive(Debug)]
pub enum AlgorithmError {
    InvalidInput,
    OutOfMemory,
    Timeout,
    InternalError,
}
```

---

**Próximo Módulo**: [Módulo 10: Projeto de Pesquisa Final](../modulo-10-projeto-pesquisa/README.md)

---

**Desenvolvido com ❤️ para a comunidade acadêmica brasileira**

*ETEC Bento Quirino - Curso Completo de Rust para Sistemas Embarcados*
