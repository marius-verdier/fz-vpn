#[derive(EnumAsInner, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Message {
    RequestINLogin(LoginInf),
    RequestSTARTConnection(),
    RequestTERMINATEConnection(),
    RequestOUTLogin(LoginInf),
    RequestFailure(String),
    RequestSuccess(),
}