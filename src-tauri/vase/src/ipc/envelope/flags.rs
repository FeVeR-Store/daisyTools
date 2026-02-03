use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Default)]
    pub struct Flags: u8 {
        const NONE       = 0b0000_0000;
        const COMPRESSED = 0b0000_0001; // 压缩
        const ENCRYPTED  = 0b0000_0010; // 加密（AEAD）
        const BATCH      = 0b0000_0100; // 批量帧（payload 内含多条子消息）
        const KEEP_ALIVE = 0b0000_1000;
        // 预留位（将来可新增协议升级、优先级等）
        const _RSV_4     = 0b0001_0000;
        const _RSV_5     = 0b0010_0000;
        const _RSV_6     = 0b0100_0000;
        const _RSV_7     = 0b1000_0000;
    }
}

pub const FLAG_LEN: usize = 1;
