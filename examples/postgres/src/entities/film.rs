# ! [doc = " SeaORM Entity. Generated by sea-orm-codegen 0.9.1"] use sea_orm :: entity :: prelude :: * ; use super :: sea_orm_active_enums :: MpaaRating ; # [derive (Copy , Clone , Default , Debug , DeriveEntity)] pub struct Entity ; impl EntityName for Entity { fn table_name (& self) -> & str { "film" } } # [derive (Clone , Debug , PartialEq , DeriveModel , DeriveActiveModel , async_graphql :: SimpleObject , seaography_derive :: Filter)] # [sea_orm (table_name = "film")] # [graphql (complex)] # [graphql (name = "Film")] pub struct Model { pub film_id : i32 , pub title : String , pub description : Option < String > , pub release_year : Option < i32 > , pub language_id : i16 , pub original_language_id : Option < i16 > , pub rental_duration : i16 , pub rental_rate : Decimal , pub length : Option < i16 > , pub replacement_cost : Decimal , pub rating : Option < MpaaRating > , pub last_update : DateTime , pub special_features : Option < String > , pub fulltext : String , } # [derive (Copy , Clone , Debug , EnumIter , DeriveColumn)] pub enum Column { FilmId , Title , Description , ReleaseYear , LanguageId , OriginalLanguageId , RentalDuration , RentalRate , Length , ReplacementCost , Rating , LastUpdate , SpecialFeatures , Fulltext , } # [derive (Copy , Clone , Debug , EnumIter , DerivePrimaryKey)] pub enum PrimaryKey { FilmId , } impl PrimaryKeyTrait for PrimaryKey { type ValueType = i32 ; fn auto_increment () -> bool { true } } # [derive (Copy , Clone , Debug , EnumIter)] pub enum Relation { Language2 , Language1 , FilmActor , FilmCategory , Inventory , } impl ColumnTrait for Column { type EntityName = Entity ; fn def (& self) -> ColumnDef { match self { Self :: FilmId => ColumnType :: Integer . def () , Self :: Title => ColumnType :: String (Some (255u32)) . def () , Self :: Description => ColumnType :: Text . def () . null () , Self :: ReleaseYear => ColumnType :: Integer . def () . null () , Self :: LanguageId => ColumnType :: SmallInteger . def () , Self :: OriginalLanguageId => ColumnType :: SmallInteger . def () . null () , Self :: RentalDuration => ColumnType :: SmallInteger . def () , Self :: RentalRate => ColumnType :: Decimal (Some ((4u32 , 2u32))) . def () , Self :: Length => ColumnType :: SmallInteger . def () . null () , Self :: ReplacementCost => ColumnType :: Decimal (Some ((5u32 , 2u32))) . def () , Self :: Rating => MpaaRating :: db_type () . null () , Self :: LastUpdate => ColumnType :: DateTime . def () , Self :: SpecialFeatures => ColumnType :: Custom ("array" . to_owned ()) . def () . null () , Self :: Fulltext => ColumnType :: Custom ("tsvector" . to_owned ()) . def () , } } } # [seaography_derive :: relation] impl RelationTrait for Relation { fn def (& self) -> RelationDef { match self { Self :: Language2 => Entity :: belongs_to (super :: language :: Entity) . from (Column :: LanguageId) . to (super :: language :: Column :: LanguageId) . into () , Self :: Language1 => Entity :: belongs_to (super :: language :: Entity) . from (Column :: OriginalLanguageId) . to (super :: language :: Column :: LanguageId) . into () , Self :: FilmActor => Entity :: has_many (super :: film_actor :: Entity) . into () , Self :: FilmCategory => Entity :: has_many (super :: film_category :: Entity) . into () , Self :: Inventory => Entity :: has_many (super :: inventory :: Entity) . into () , } } } impl Related < super :: film_actor :: Entity > for Entity { fn to () -> RelationDef { Relation :: FilmActor . def () } } impl Related < super :: film_category :: Entity > for Entity { fn to () -> RelationDef { Relation :: FilmCategory . def () } } impl Related < super :: inventory :: Entity > for Entity { fn to () -> RelationDef { Relation :: Inventory . def () } } impl ActiveModelBehavior for ActiveModel { }