use crate::organism::Organism;

pub trait Vitality {
    async fn sense();

    async fn flow();

    async fn extend();

    async fn pulse() {
        loop {
            Self::sense().await;
            Self::flow().await;
            Self::extend().await;
        }
    }
}

impl Vitality for Organism {
    async fn sense() {
        todo!()
    }

    async fn flow() {
        todo!()
    }

    async fn extend() {
        todo!()
    }
}

pub fn assert_vitality_impl<T: Vitality>() {}
