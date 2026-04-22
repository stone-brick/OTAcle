//! Think 模块类型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 决策日志消息（由 Python 端发送）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionLog {
    /// 训练步数
    pub step: u64,
    /// 自定义字段（可选）
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// Think 模块配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkConfig {
    /// 环形缓冲区最大数据点数
    #[serde(default = "default_max_data_points")]
    pub max_data_points: u32,
    /// 要显示的字段列表
    #[serde(default)]
    pub display_fields: Vec<DisplayField>,
}

impl Default for ThinkConfig {
    fn default() -> Self {
        Self {
            max_data_points: default_max_data_points(),
            display_fields: vec![DisplayField {
                name: "step".to_string(),
                title: "训练步数".to_string(),
                chart_type: ChartType::Line,
            }],
        }
    }
}

fn default_max_data_points() -> u32 {
    1000
}

/// 单个显示字段配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayField {
    /// 字段名（对应 DecisionLog 的字段名）
    pub name: String,
    /// 显示标题
    pub title: String,
    /// 图表类型：line, bar, area, gauge
    pub chart_type: ChartType,
}

/// 图表类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
    Line,
    Bar,
    Area,
    Gauge,
}

/// Think 状态（用于 think_get_status 返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkStatus {
    pub running: bool,
    pub logs_count: u64,
    pub connected: bool,
    pub messages_received: u64,
    pub uptime_seconds: u64,
}