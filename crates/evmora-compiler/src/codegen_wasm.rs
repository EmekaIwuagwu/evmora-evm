use crate::ir::IrProgram;

pub struct WasmCodegen;

impl WasmCodegen {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, ir: &IrProgram) -> Vec<u8> {
        // Generate WASM bytecode from IR
        // WASM binary format: https://webassembly.github.io/spec/core/binary/index.html
        
        let mut wasm = Vec::new();
        
        // Magic number: \0asm
        wasm.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]);
        
        // Version: 1
        wasm.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
        
        // Type section (section 1)
        wasm.push(0x01); // Section ID
        let type_section = self.generate_type_section();
        self.write_vector(&mut wasm, &type_section);
        
        // Function section (section 3)
        wasm.push(0x03); // Section ID
        let func_section = vec![0x01, 0x00]; // 1 function, type 0
        self.write_vector(&mut wasm, &func_section);
        
        // Export section (section 7)
        wasm.push(0x07); // Section ID
        let export_section = self.generate_export_section();
        self.write_vector(&mut wasm, &export_section);
        
        // Code section (section 10)
        wasm.push(0x0A); // Section ID
        let code_section = self.generate_code_section(ir);
        self.write_vector(&mut wasm, &code_section);
        
        wasm
    }
    
    fn write_vector(&self, dest: &mut Vec<u8>, data: &[u8]) {
        self.write_uleb128(dest, data.len() as u32);
        dest.extend_from_slice(data);
    }
    
    fn write_uleb128(&self, dest: &mut Vec<u8>, mut value: u32) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            dest.push(byte);
            if value == 0 {
                break;
            }
        }
    }
    
    fn generate_type_section(&self) -> Vec<u8> {
        let mut section = Vec::new();
        section.push(0x01); // 1 type
        section.push(0x60); // func type
        section.push(0x00); // 0 params
        section.push(0x00); // 0 results
        section
    }
    
    fn generate_export_section(&self) -> Vec<u8> {
        let mut section = Vec::new();
        section.push(0x01); // 1 export
        // Export name "deploy"
        section.push(0x06); // name length
        section.extend_from_slice(b"deploy");
        section.push(0x00); // export kind: function
        section.push(0x00); // function index 0
        section
    }
    
    fn generate_code_section(&self, ir: &IrProgram) -> Vec<u8> {
        let mut section = Vec::new();
        section.push(0x01); // 1 code entry
        
        let mut func_body = Vec::new();
        func_body.push(0x00); // 0 locals
        
        // Generate WASM instructions from IR
        for stmt in &ir.statements {
            match stmt {
                crate::ir::IrStatement::Push(val) => {
                    // i64.const
                    func_body.push(0x42);
                    let v = val.low_u64() as i64;
                    self.write_sleb128(&mut func_body, v);
                }
                crate::ir::IrStatement::Add => {
                    func_body.push(0x7C); // i64.add
                }
                crate::ir::IrStatement::Sub => {
                    func_body.push(0x7D); // i64.sub
                }
                crate::ir::IrStatement::Pop => {
                    func_body.push(0x1A); // drop
                }
                _ => {
                    // Handle other IR statements
                }
            }
        }
        
        // End function
        func_body.push(0x0B);
        
        // Write function body size and body
        self.write_uleb128(&mut section, func_body.len() as u32);
        section.extend_from_slice(&func_body);
        
        section
    }
    
    fn write_sleb128(&self, dest: &mut Vec<u8>, mut value: i64) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            let sign_bit = (byte & 0x40) != 0;
            if (value == 0 && !sign_bit) || (value == -1 && sign_bit) {
                dest.push(byte);
                break;
            } else {
                dest.push(byte | 0x80);
            }
        }
    }
}

