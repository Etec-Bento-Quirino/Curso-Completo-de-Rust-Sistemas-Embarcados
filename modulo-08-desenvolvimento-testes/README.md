# 🧪 Módulo 8: Desenvolvimento e Testes

## 🎯 **Objetivos de Aprendizagem Acadêmica**

Ao final deste módulo, você será capaz de:
- ✅ Implementar debugging em sistemas embarcados
- ✅ Desenvolver testes unitários e de integração
- ✅ Implementar hardware-in-the-loop testing
- ✅ Configurar continuous integration para embarcados
- ✅ Criar ferramentas de análise e profiling
- ✅ Desenvolver metodologias de teste para sistemas críticos

## 📚 **Conteúdo Teórico Acadêmico**

### **8.1 Debugging em Sistemas Embarcados**

#### **Sistema de Debug**
```rust
// debug_system.rs
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use core::fmt::Write;

pub struct DebugSystem {
    enabled: AtomicBool,
    log_level: AtomicU32,
    buffer: [u8; 1024],
    buffer_index: AtomicU32,
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Error = 0,
    Warning = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

impl DebugSystem {
    pub const fn new() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            log_level: AtomicU32::new(LogLevel::Info as u32),
            buffer: [0; 1024],
            buffer_index: AtomicU32::new(0),
        }
    }
    
    pub fn log(&self, level: LogLevel, message: &str) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }
        
        if level as u32 > self.log_level.load(Ordering::Relaxed) {
            return;
        }
        
        self.write_to_buffer(level, message);
    }
    
    fn write_to_buffer(&self, level: LogLevel, message: &str) {
        let mut index = self.buffer_index.load(Ordering::Relaxed);
        let buffer_len = self.buffer.len() as u32;
        
        // Escrever timestamp
        let timestamp = self.get_timestamp();
        let timestamp_str = format!("[{}]", timestamp);
        
        for byte in timestamp_str.bytes() {
            if index < buffer_len {
                unsafe {
                    self.buffer.as_ptr().add(index as usize).write(byte);
                }
                index += 1;
            }
        }
        
        // Escrever nível
        let level_str = match level {
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARN ",
            LogLevel::Info => "INFO ",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        };
        
        for byte in level_str.bytes() {
            if index < buffer_len {
                unsafe {
                    self.buffer.as_ptr().add(index as usize).write(byte);
                }
                index += 1;
            }
        }
        
        // Escrever mensagem
        for byte in message.bytes() {
            if index < buffer_len {
                unsafe {
                    self.buffer.as_ptr().add(index as usize).write(byte);
                }
                index += 1;
            }
        }
        
        // Nova linha
        if index < buffer_len {
            unsafe {
                self.buffer.as_ptr().add(index as usize).write(b'\n');
            }
            index += 1;
        }
        
        self.buffer_index.store(index, Ordering::Relaxed);
    }
    
    fn get_timestamp(&self) -> u32 {
        // Implementar com timer do sistema
        0
    }
    
    pub fn dump_logs(&self) -> &str {
        let index = self.buffer_index.load(Ordering::Relaxed) as usize;
        unsafe {
            core::str::from_utf8_unchecked(&self.buffer[..index])
        }
    }
}

// Macro para logging fácil
#[macro_export]
macro_rules! log {
    ($level:ident, $($arg:tt)*) => {
        DEBUG_SYSTEM.log(LogLevel::$level, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log!(Error, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log!(Info, $($arg)*);
    };
}

static DEBUG_SYSTEM: DebugSystem = DebugSystem::new();
```

### **8.2 Testes Unitários**

```rust
// unit_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicU32, Ordering};
    
    #[test]
    fn test_sensor_reading() {
        let mut sensor = TemperatureSensor::new();
        
        // Teste com valor válido
        let result = sensor.read_temperature();
        assert!(result.is_ok());
        
        let temperature = result.unwrap();
        assert!(temperature >= -40.0 && temperature <= 125.0);
    }
    
    #[test]
    fn test_can_communication() {
        let mut can_controller = CANController::new();
        
        let test_data = [0x01, 0x02, 0x03, 0x04];
        let result = can_controller.send_data(0x100, &test_data);
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_interrupt_handling() {
        static INTERRUPT_COUNT: AtomicU32 = AtomicU32::new(0);
        
        // Simular interrupção
        INTERRUPT_COUNT.fetch_add(1, Ordering::Relaxed);
        
        assert_eq!(INTERRUPT_COUNT.load(Ordering::Relaxed), 1);
    }
    
    #[test]
    fn test_memory_allocation() {
        let allocator = EmbeddedAllocator::new(core::ptr::null_mut(), 1024);
        
        let layout = core::alloc::Layout::new::<u32>();
        unsafe {
            let ptr = allocator.alloc(layout);
            assert!(!ptr.is_null());
            
            // Escrever e ler dados
            ptr.write(0x12345678);
            assert_eq!(ptr.read(), 0x12345678);
            
            allocator.dealloc(ptr, layout);
        }
    }
}
```

### **8.3 Hardware-in-the-Loop Testing**

```rust
// hil_testing.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct HILTestSystem {
    hardware_simulator: Arc<Mutex<HardwareSimulator>>,
    test_cases: Vec<TestCase>,
}

pub struct HardwareSimulator {
    gpio_states: [bool; 32],
    adc_values: [u16; 8],
    timer_value: u32,
}

impl HardwareSimulator {
    pub fn new() -> Self {
        Self {
            gpio_states: [false; 32],
            adc_values: [0; 8],
            timer_value: 0,
        }
    }
    
    pub fn set_gpio(&mut self, pin: usize, state: bool) {
        if pin < 32 {
            self.gpio_states[pin] = state;
        }
    }
    
    pub fn get_gpio(&self, pin: usize) -> bool {
        if pin < 32 {
            self.gpio_states[pin]
        } else {
            false
        }
    }
    
    pub fn set_adc_value(&mut self, channel: usize, value: u16) {
        if channel < 8 {
            self.adc_values[channel] = value;
        }
    }
    
    pub fn get_adc_value(&self, channel: usize) -> u16 {
        if channel < 8 {
            self.adc_values[channel]
        } else {
            0
        }
    }
    
    pub fn tick_timer(&mut self) {
        self.timer_value = self.timer_value.wrapping_add(1);
    }
}

pub struct TestCase {
    name: String,
    setup: Box<dyn Fn(&mut HardwareSimulator) + Send + Sync>,
    test: Box<dyn Fn(&HardwareSimulator) -> TestResult + Send + Sync>,
    teardown: Box<dyn Fn(&mut HardwareSimulator) + Send + Sync>,
}

#[derive(Debug)]
pub enum TestResult {
    Pass,
    Fail(String),
}

impl HILTestSystem {
    pub fn new() -> Self {
        Self {
            hardware_simulator: Arc::new(Mutex::new(HardwareSimulator::new())),
            test_cases: Vec::new(),
        }
    }
    
    pub fn add_test_case(&mut self, test_case: TestCase) {
        self.test_cases.push(test_case);
    }
    
    pub fn run_all_tests(&self) -> Vec<TestReport> {
        let mut reports = Vec::new();
        
        for test_case in &self.test_cases {
            let report = self.run_test(test_case);
            reports.push(report);
        }
        
        reports
    }
    
    fn run_test(&self, test_case: &TestCase) -> TestReport {
        let mut simulator = self.hardware_simulator.lock().unwrap();
        
        // Setup
        (test_case.setup)(&mut simulator);
        
        // Executar teste
        let result = (test_case.test)(&simulator);
        
        // Teardown
        (test_case.teardown)(&mut simulator);
        
        TestReport {
            name: test_case.name.clone(),
            result,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    pub fn run_stress_test(&self, duration: Duration) -> StressTestResult {
        let start_time = std::time::Instant::now();
        let mut iterations = 0;
        let mut errors = 0;
        
        while start_time.elapsed() < duration {
            for test_case in &self.test_cases {
                let report = self.run_test(test_case);
                iterations += 1;
                
                if let TestResult::Fail(_) = report.result {
                    errors += 1;
                }
            }
        }
        
        StressTestResult {
            duration: start_time.elapsed(),
            iterations,
            errors,
            error_rate: if iterations > 0 { errors as f32 / iterations as f32 } else { 0.0 },
        }
    }
}

#[derive(Debug)]
pub struct TestReport {
    pub name: String,
    pub result: TestResult,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct StressTestResult {
    pub duration: Duration,
    pub iterations: u32,
    pub errors: u32,
    pub error_rate: f32,
}
```

## 💻 **Exemplos Práticos Acadêmicos**

### **Exemplo 1: Suite de Testes Automatizada**

**Objetivo de Pesquisa**: Sistema de testes automatizado para sistemas embarcados críticos

```rust
// automated_test_suite.rs
use std::collections::HashMap;

pub struct AutomatedTestSuite {
    test_modules: HashMap<String, TestModule>,
    test_results: Vec<TestResult>,
    coverage_analyzer: CoverageAnalyzer,
}

pub struct TestModule {
    name: String,
    tests: Vec<Box<dyn EmbeddedTest>>,
    coverage_target: f32,
}

pub trait EmbeddedTest {
    fn name(&self) -> &str;
    fn run(&mut self) -> TestResult;
    fn get_coverage(&self) -> f32;
}

pub struct CoverageAnalyzer {
    line_coverage: HashMap<String, Vec<bool>>,
    branch_coverage: HashMap<String, Vec<bool>>,
    function_coverage: HashMap<String, bool>,
}

impl AutomatedTestSuite {
    pub fn new() -> Self {
        Self {
            test_modules: HashMap::new(),
            test_results: Vec::new(),
            coverage_analyzer: CoverageAnalyzer::new(),
        }
    }
    
    pub fn add_test_module(&mut self, module: TestModule) {
        self.test_modules.insert(module.name.clone(), module);
    }
    
    pub fn run_all_tests(&mut self) -> TestSuiteResult {
        let mut total_tests = 0;
        let mut passed_tests = 0;
        let mut failed_tests = 0;
        let mut total_coverage = 0.0;
        
        for (module_name, module) in &mut self.test_modules {
            println!("Running tests for module: {}", module_name);
            
            for test in &mut module.tests {
                total_tests += 1;
                let result = test.run();
                
                match result {
                    TestResult::Pass => {
                        passed_tests += 1;
                        println!("  ✓ {} - PASSED", test.name());
                    }
                    TestResult::Fail(reason) => {
                        failed_tests += 1;
                        println!("  ✗ {} - FAILED: {}", test.name(), reason);
                    }
                }
                
                total_coverage += test.get_coverage();
                self.test_results.push(result);
            }
        }
        
        let average_coverage = if total_tests > 0 {
            total_coverage / total_tests as f32
        } else {
            0.0
        };
        
        TestSuiteResult {
            total_tests,
            passed_tests,
            failed_tests,
            success_rate: if total_tests > 0 {
                passed_tests as f32 / total_tests as f32 * 100.0
            } else {
                0.0
            },
            coverage: average_coverage,
        }
    }
    
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Test Suite Report\n\n");
        report.push_str(&format!("Total Tests: {}\n", self.test_results.len()));
        
        let passed = self.test_results.iter()
            .filter(|r| matches!(r, TestResult::Pass))
            .count();
        
        let failed = self.test_results.len() - passed;
        
        report.push_str(&format!("Passed: {}\n", passed));
        report.push_str(&format!("Failed: {}\n", failed));
        report.push_str(&format!("Success Rate: {:.1}%\n", 
            passed as f32 / self.test_results.len() as f32 * 100.0));
        
        report
    }
}

#[derive(Debug)]
pub struct TestSuiteResult {
    pub total_tests: u32,
    pub passed_tests: u32,
    pub failed_tests: u32,
    pub success_rate: f32,
    pub coverage: f32,
}

impl CoverageAnalyzer {
    pub fn new() -> Self {
        Self {
            line_coverage: HashMap::new(),
            branch_coverage: HashMap::new(),
            function_coverage: HashMap::new(),
        }
    }
    
    pub fn add_line_coverage(&mut self, file: String, line: usize, covered: bool) {
        self.line_coverage.entry(file)
            .or_insert_with(Vec::new)
            .resize(line + 1, false);
        
        if let Some(coverage) = self.line_coverage.get_mut(&file) {
            coverage[line] = covered;
        }
    }
    
    pub fn calculate_line_coverage(&self, file: &str) -> f32 {
        if let Some(coverage) = self.line_coverage.get(file) {
            let covered_lines = coverage.iter().filter(|&&covered| covered).count();
            covered_lines as f32 / coverage.len() as f32 * 100.0
        } else {
            0.0
        }
    }
    
    pub fn calculate_total_coverage(&self) -> f32 {
        let mut total_lines = 0;
        let mut covered_lines = 0;
        
        for coverage in self.line_coverage.values() {
            total_lines += coverage.len();
            covered_lines += coverage.iter().filter(|&&covered| covered).count();
        }
        
        if total_lines > 0 {
            covered_lines as f32 / total_lines as f32 * 100.0
        } else {
            0.0
        }
    }
}
```

---

**Próximo Módulo**: [Módulo 9: Análise de Performance](../modulo-09-analise-performance/README.md)

---

**Desenvolvido com ❤️ para a comunidade acadêmica brasileira**

*ETEC Bento Quirino - Curso Completo de Rust para Sistemas Embarcados*
