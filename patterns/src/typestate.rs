use std::marker::PhantomData;

// ==========================================
// 1. 定义表示“状态”的空结构体（Zero-Sized Types）
// ==========================================
pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

// ==========================================
// 2. 泛型结构体：将状态作为类型参数嵌入
// ==========================================
pub struct HttpConnection<State> {
    url: String,
    token: Option<String>,
    // PhantomData 用于告知编译器该结构体拥有 State 类型参数，但不占用实际物理内存
    _state: PhantomData<State>,
}

// ==========================================
// 3. 针对不同状态实现专属的方法（编译期逻辑隔离）
// ==========================================

// 阶段 A: 未连接状态下，只能调用 connect 方法
impl HttpConnection<Disconnected> {
    pub fn new(url: impl Into<String>) -> Self {
        HttpConnection {
            url: url.into(),
            token: None,
            _state: PhantomData,
        }
    }

    // 状态转换：Disconnected -> Connected (消费 self，返回新类型)
    pub fn connect(self) -> HttpConnection<Connected> {
        println!("正在连接到 {}", self.url);
        HttpConnection {
            url: self.url,
            token: None,
            _state: PhantomData,
        }
    }
}

// 阶段 B: 已连接状态下，只能调用 authenticate 方法
impl HttpConnection<Connected> {
    // 状态转换：Connected -> Authenticated
    pub fn authenticate(mut self, token: impl Into<String>) -> HttpConnection<Authenticated> {
        println!("校验 Token 并完成鉴权...");
        self.token = Some(token.into());
        HttpConnection {
            url: self.url,
            token: self.token,
            _state: PhantomData,
        }
    }
}

// 阶段 C: 只有鉴权成功后，才能调用 send_request 发送请求
impl HttpConnection<Authenticated> {
    pub fn send_request(&self, payload: &str) {
        println!(
            "发送请求到 [{}]，Token: [{}]，数据: {}",
            self.url,
            self.token.as_ref().unwrap(),
            payload
        );
    }
}
