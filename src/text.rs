pub trait Formattable
{
    fn to_titlecase(&self) -> String;
}

impl<T> Formattable for T
where
    T: ToString,
{
    fn to_titlecase(&self) -> String
    {
        let source = self.to_string();

        if source.len() < 2
        {
            source.to_uppercase()
        }
        else
        {
            let (first, rest) = source.split_at(1);
            let mut formatted = String::with_capacity(source.len());
            formatted.push_str(&first.to_uppercase());
            formatted.push_str(rest);

            formatted
        }
    }
}
