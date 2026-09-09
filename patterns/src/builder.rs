use time;

#[derive(Debug, Clone)]
pub struct PhoneNumberE164(pub String);

#[derive(Debug, Clone)]
pub struct Details {
    pub given_name: String,
    pub preferred_name: Option<String>,
    pub middle_name: Option<String>,
    pub family_name: String,
    pub mobile_phone: Option<PhoneNumberE164>,
    pub date_of_birth: time::Date,
    pub last_seen: Option<time::OffsetDateTime>,
}

/// 每次调用都消耗自身，同一个builder不能通过多次调用build来构造对象
pub struct DetailsBuilder(Details);

impl DetailsBuilder {
    /// Start building a new [`Details`] object.
    /// 开始构造一个新的 [`Details`] 对象
    pub fn new(given_name: &str, family_name: &str, date_of_birth: time::Date) -> Self {
        Self(Details {
            given_name: given_name.to_owned(),
            preferred_name: None,
            middle_name: None,
            family_name: family_name.to_owned(),
            mobile_phone: None,
            date_of_birth,
            last_seen: None,
        })
    }

    pub fn middle_name(mut self, s: &str) -> Self {
        self.0.middle_name = Some(s.to_owned());
        self
    }

    pub fn preferred_name(mut self, s: &str) -> Self {
        self.0.preferred_name = Some(s.to_owned());
        self
    }

    pub fn just_seen(mut self) -> Self {
        self.0.last_seen = Some(time::OffsetDateTime::now_utc());
        self
    }

    pub fn build(self) -> Details {
        self.0
    }
}

/// 构造的方法得到可变引用，同时返回可变引用，最后构造时也不消耗对象，
/// 这样同一个构造器可以用来构造多个对象
pub struct DetailsBuilder2(Details);

impl DetailsBuilder2 {
    pub fn new(given_name: &str, family_name: &str, date_of_birth: time::Date) -> Self {
        Self(Details {
            given_name: given_name.to_owned(),
            preferred_name: None,
            middle_name: None,
            family_name: family_name.to_owned(),
            mobile_phone: None,
            date_of_birth,
            last_seen: None,
        })
    }

    pub fn middle_name(&mut self, s: &str) -> &mut Self {
        self.0.middle_name = Some(s.to_owned());
        self
    }

    pub fn preferred_name(&mut self, s: &str) -> &mut Self {
        self.0.preferred_name = Some(s.to_owned());
        self
    }

    pub fn just_seen(&mut self) -> &mut Self {
        self.0.last_seen = Some(time::OffsetDateTime::now_utc());
        self
    }

    pub fn build(&self) -> Details {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let _bob = DetailsBuilder::new(
            "Robert",
            "Builder",
            time::Date::from_calendar_date(1998, time::Month::November, 28).unwrap(),
        )
        .middle_name("the")
        .preferred_name("Bob")
        .just_seen()
        .build();
    }

    #[test]
    fn test2() {
        let mut builder = DetailsBuilder2::new(
            "Robert",
            "Builder",
            time::Date::from_calendar_date(1998, time::Month::November, 28).unwrap(),
        );
        builder.middle_name("the").just_seen();

        if true {
            builder.preferred_name("Bob"); // builder没被消耗，不用重复赋值： builder = ...
        }
        let _bob = builder.build();
    }
}
