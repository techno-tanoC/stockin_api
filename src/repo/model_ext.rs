use anyhow::Result;

pub trait ModelExt {
    type Target;

    fn convert(self) -> Result<Self::Target>;
}

impl<T> ModelExt for Option<T>
where
    T: ModelExt,
{
    type Target = Option<T::Target>;

    fn convert(self) -> Result<Self::Target> {
        self.map(ModelExt::convert).transpose()
    }
}

impl<T> ModelExt for Vec<T>
where
    T: ModelExt,
{
    type Target = Vec<T::Target>;

    fn convert(self) -> Result<Self::Target> {
        self.into_iter().map(ModelExt::convert).collect()
    }
}
