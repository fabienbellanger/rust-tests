table! {
    ApiConnection (id) {
        id -> Integer,
        name -> Varchar,
        authKey -> Varchar,
        active -> Smallint,
        datetimeCreated -> Datetime,
    }
}

table! {
    AppliVersion (id) {
        id -> Integer,
        version -> Varchar,
    }
}

table! {
    Badge (id) {
        id -> Integer,
        name -> Varchar,
        image -> Varchar,
    }
}

table! {
    CocaCola (id) {
        id -> Integer,
        code -> Varchar,
        used -> Smallint,
    }
}

table! {
    DeliveryAddress (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        streetType -> Varchar,
        streetName -> Varchar,
        zipcode -> Varchar,
        city -> Varchar,
        deliveryDuration -> Integer,
        mapCode -> Varchar,
        advertMapCode -> Varchar,
        mailingName -> Varchar,
        isDeliverable -> Smallint,
    }
}

table! {
    Flyers (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        name -> Varchar,
        link -> Varchar,
        showed -> Smallint,
    }
}

table! {
    Job (id) {
        id -> Integer,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        duration -> Varchar,
        exp -> Varchar,
        mission -> Longtext,
        email -> Varchar,
        date -> Date,
        shop_id -> Nullable<Integer>,
    }
}

table! {
    JobCvExp (id) {
        id -> Integer,
        spontane_id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Varchar,
        year -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
    }
}

table! {
    JobSpontaneous (id) {
        id -> Integer,
        phone -> Varchar,
        email -> Varchar,
        zipcode -> Nullable<Integer>,
        birth -> Nullable<Date>,
        date_now -> Date,
        current -> Varchar,
        contrat -> Varchar,
        shop_id -> Nullable<Integer>,
        civility -> Varchar,
        lastname -> Varchar,
        firstname -> Varchar,
        city -> Varchar,
        wanted -> Varchar,
        horair -> Nullable<Varchar>,
    }
}

table! {
    Navigation (id) {
        id -> Integer,
        name -> Varchar,
        link -> Varchar,
        target -> Smallint,
        position -> Integer,
    }
}

table! {
    NewsletterUser (id) {
        id -> Integer,
        email -> Varchar,
        acceptOffers -> Smallint,
        acceptPartnersOffers -> Smallint,
    }
}

table! {
    Orangina (id) {
        id -> Integer,
        lastname -> Varchar,
        firstname -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        birthDay -> Nullable<Date>,
        zipcode -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
    }
}

table! {
    PayboxSecurity (id) {
        id -> Integer,
        identifier -> Varchar,
        privateKey -> Varchar,
        publicKey -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Bigint,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    PromotionalOffers (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        name -> Varchar,
        image -> Nullable<Varchar>,
        link -> Varchar,
        active -> Smallint,
        dateBegin -> Date,
        dateEnd -> Nullable<Date>,
        targetBlank -> Nullable<Smallint>,
        position -> Integer,
        isPromo -> Nullable<Bool>,
    }
}

table! {
    PushHistory (id) {
        id -> Integer,
        tokenId -> Varchar,
        platform -> Varchar,
        latitude -> Double,
        longitude -> Double,
        date -> Datetime,
        messageId -> Nullable<Integer>,
        nearShopId -> Nullable<Integer>,
        shopId -> Nullable<Integer>,
        state -> Integer,
        statId -> Nullable<Integer>,
    }
}

table! {
    PushMessage (id) {
        id -> Integer,
        messageObject -> Varchar,
        message -> Varchar,
    }
}

table! {
    PushStatistics (id) {
        id -> Integer,
        shopName -> Varchar,
        #[sql_name = "type"]
        type_ -> Smallint,
        targetsNumber -> Integer,
        date -> Nullable<Datetime>,
        shopId -> Nullable<Integer>,
        messageId -> Nullable<Integer>,
        state -> Integer,
        actifTokens -> Integer,
        inactifTokens -> Integer,
        invalidTokens -> Integer,
    }
}

table! {
    PushTokens (token) {
        token -> Varchar,
        platform -> Varchar,
        latitude -> Nullable<Double>,
        longitude -> Nullable<Double>,
        date -> Date,
        shopId -> Nullable<Integer>,
        actif -> Smallint,
        acceptPush -> Smallint,
    }
}

table! {
    Settings (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        dateBegin -> Date,
    }
}

table! {
    Shop (id) {
        id -> Integer,
        name -> Varchar,
        acctKey -> Nullable<Varchar>,
        lopaId -> Nullable<Varchar>,
        address -> Varchar,
        zipcode -> Varchar,
        city -> Varchar,
        region -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        phone -> Varchar,
        fax -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        openingHours -> Varchar,
        otherZipcodes -> Nullable<Varchar>,
        hasOnlineCommand -> Smallint,
        longitude -> Nullable<Double>,
        latitude -> Nullable<Double>,
        zoom -> Nullable<Integer>,
        shopCode -> Nullable<Varchar>,
        visible -> Smallint,
        hasOnlinePayment -> Smallint,
        onlinePaymentType -> Nullable<Varchar>,
        facebookTracking -> Nullable<Longtext>,
        fidelite2_account_id -> Nullable<Varchar>,
        paid_at_first_command -> Bool,
    }
}

table! {
    ShopAtos (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        merchant_id -> Varchar,
        merchant_country -> Varchar,
        payment_means -> Varchar,
        currency_code -> Varchar,
        active -> Bool,
        language -> Varchar,
    }
}

table! {
    ShopAtosTransaction (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        code -> Smallint,
        error -> Varchar,
        merchant_id -> Varchar,
        amount -> Integer,
        authorisation_id -> Varchar,
        complementary_code -> Varchar,
        complementary_info -> Varchar,
        currency_code -> Varchar,
        customer_ip_address -> Varchar,
        payment_certificate -> Varchar,
        response_code -> Varchar,
        transaction_id -> Varchar,
        transmission_date -> Varchar,
        pdsa_id -> Integer,
    }
}

table! {
    ShopCmcic (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        login -> Varchar,
        currency -> Varchar,
        version -> Varchar,
        tpe -> Varchar,
        language -> Varchar,
        cle -> Varchar,
    }
}

table! {
    ShopCmcicTransaction (id) {
        id -> Integer,
        shop_id -> Integer,
        user_id -> Integer,
        reference -> Varchar,
        montant -> Varchar,
        date -> Varchar,
        version -> Varchar,
        codeRetour -> Varchar,
        cvx -> Varchar,
        vld -> Varchar,
        brand -> Varchar,
        status3ds -> Varchar,
        numAuto -> Varchar,
        motifRefus -> Varchar,
        origineCB -> Varchar,
        binCB -> Varchar,
        hpanCB -> Varchar,
        ipClient -> Varchar,
        origineTR -> Varchar,
        veres -> Varchar,
        pares -> Varchar,
        pdsa_id -> Integer,
    }
}

table! {
    ShopGroup (id) {
        id -> Integer,
        name -> Varchar,
        intranetAccess -> Smallint,
        backofficeAccess -> Smallint,
        ShopGroup_id -> Nullable<Integer>,
    }
}

table! {
    ShopLydia (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        vendor_token -> Varchar,
        vendor_id -> Varchar,
    }
}

table! {
    ShopLydiaTransaction (id) {
        id -> Integer,
        shop_id -> Integer,
        user_id -> Integer,
        pdsaId -> Integer,
        montant -> Varchar,
        orderId -> Varchar,
        currency -> Varchar,
        requestId -> Varchar,
        signed -> Varchar,
        transactionIdent -> Varchar,
        vendorToken -> Varchar,
        sign -> Varchar,
    }
}

table! {
    ShopOgone (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        merchant_id -> Varchar,
        key -> Varchar,
    }
}

table! {
    ShopOgoneTransaction (id) {
        id -> Integer,
        shop_id -> Integer,
        user_id -> Integer,
        pdsaId -> Integer,
        amount -> Varchar,
        orderId -> Varchar,
        currency -> Varchar,
        paymentMean -> Varchar,
        acceptance -> Varchar,
        status -> Varchar,
        cardNo -> Varchar,
        name -> Varchar,
        date -> Varchar,
        paymentId -> Varchar,
        error -> Varchar,
        brand -> Varchar,
        eci -> Varchar,
        ip -> Varchar,
        shaSign -> Varchar,
    }
}

table! {
    ShopParameters (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        acceptCreditCard -> Smallint,
        acceptElectronCard -> Smallint,
        acceptCheque -> Smallint,
        acceptLunchVoucher -> Smallint,
        acceptTravellerCheck -> Smallint,
        halal -> Smallint,
        minPriceDelivery -> Double,
        lunchServiceDeliveryBegin -> Nullable<Varchar>,
        lunchServiceDeliveryEnd -> Nullable<Varchar>,
        eveningServiceDeliveryBegin -> Nullable<Varchar>,
        eveningServiceDeliveryEnd -> Nullable<Varchar>,
        lunchServiceTakeawayBegin -> Nullable<Varchar>,
        lunchServiceTakeawayEnd -> Nullable<Varchar>,
        eveningServiceTakeawayBegin -> Nullable<Varchar>,
        eveningServiceTakeawayEnd -> Nullable<Varchar>,
        beefHalal -> Smallint,
        chickenHalal -> Smallint,
        merguezHalal -> Smallint,
        monday -> Varchar,
        tuesday -> Varchar,
        wednesday -> Varchar,
        thursday -> Varchar,
        friday -> Varchar,
        saturday -> Varchar,
        sunday -> Varchar,
    }
}

table! {
    ShopPaybox (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        site -> Varchar,
        rang -> Varchar,
        identifiant -> Varchar,
        devise -> Varchar,
        version -> Varchar,
        cle -> Varchar,
    }
}

table! {
    ShopPayboxSystem (id) {
        id -> Integer,
        shop_id -> Nullable<Integer>,
        site -> Varchar,
        rang -> Varchar,
        identifiant -> Varchar,
        devise -> Varchar,
    }
}

table! {
    ShopPayboxSystemTransaction (id) {
        id -> Integer,
        shop_id -> Integer,
        user_id -> Integer,
        pdsaId -> Integer,
        amount -> Varchar,
        orderId -> Varchar,
        transaction -> Varchar,
        abo -> Varchar,
        payment -> Varchar,
        card -> Varchar,
        payboxTransId -> Varchar,
        code -> Varchar,
        valid -> Varchar,
        ip -> Varchar,
        digest -> Varchar,
        sign -> Varchar,
    }
}

table! {
    ShopPayboxTransaction (id) {
        id -> Integer,
        shop_id -> Integer,
        user_id -> Integer,
        pdsaId -> Integer,
        #[sql_name = "type"]
        type_ -> Varchar,
        montant -> Varchar,
        dateQ -> Varchar,
        reference -> Varchar,
        numQuestion -> Varchar,
        numAppel -> Varchar,
        numTrans -> Varchar,
        autorisation -> Varchar,
        codeReponse -> Varchar,
        commentaire -> Varchar,
        typeCarte -> Varchar,
        status -> Varchar,
    }
}

table! {
    Slider (id) {
        id -> Integer,
        name -> Varchar,
        image -> Varchar,
        link -> Varchar,
        dateBegin -> Date,
        dateEnd -> Nullable<Date>,
        active -> Smallint,
        targetBlank -> Nullable<Smallint>,
        position -> Integer,
        isPromo -> Nullable<Bool>,
    }
}

table! {
    SliderChange (id) {
        id -> Integer,
        dateLastChecked -> Date,
        isChanged -> Smallint,
    }
}

table! {
    SplashScreen (id) {
        id -> Integer,
        name -> Varchar,
        dateBegin -> Date,
        dateEnd -> Nullable<Date>,
        image -> Varchar,
        active -> Smallint,
        ordering -> Smallint,
        duration -> Smallint,
    }
}

table! {
    Tasting (id) {
        id -> Integer,
        badge_id -> Nullable<Integer>,
        name -> Varchar,
        image -> Varchar,
        link -> Varchar,
        active -> Smallint,
        dateBegin -> Date,
        dateEnd -> Nullable<Date>,
        description -> Longtext,
        size -> Integer,
        position -> Integer,
        targetBlank -> Nullable<Smallint>,
    }
}

table! {
    TransactionTemp (id) {
        id -> Integer,
        value -> Longtext,
        response -> Nullable<Varchar>,
    }
}

table! {
    User (id) {
        id -> Integer,
        userName -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        password -> Varchar,
        salt -> Varchar,
        residence -> Nullable<Varchar>,
        building -> Nullable<Varchar>,
        apartment -> Nullable<Varchar>,
        floot -> Nullable<Varchar>,
        portal -> Nullable<Varchar>,
        entrance -> Nullable<Varchar>,
        lift -> Nullable<Varchar>,
        cellphone2 -> Nullable<Varchar>,
        lastname -> Varchar,
        firstname -> Varchar,
        cuacId -> Nullable<Varchar>,
        deliveryAddress_id -> Nullable<Integer>,
        streetNumber -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        acceptOffers -> Smallint,
        acceptPartnersOffers -> Smallint,
        alertComment -> Nullable<Varchar>,
        visibleComment -> Nullable<Varchar>,
        mobileCmdCnt -> Nullable<Integer>,
        hasPaidAtFirstCommand -> Bool,
    }
}

table! {
    UserShopGroup (id) {
        id -> Integer,
        User_id -> Nullable<Integer>,
        shop_id -> Nullable<Integer>,
        ShopGroup_id -> Nullable<Integer>,
    }
}

table! {
    ZipcodeCity (id) {
        id -> Integer,
        shop -> Nullable<Integer>,
        value -> Varchar,
    }
}

table! {
    Zipcodes (id) {
        id -> Integer,
        shop -> Nullable<Integer>,
        zipcode -> Varchar,
    }
}

joinable!(DeliveryAddress -> Shop (shop_id));
joinable!(Flyers -> Shop (shop_id));
joinable!(Job -> Shop (shop_id));
joinable!(JobCvExp -> JobSpontaneous (spontane_id));
joinable!(JobSpontaneous -> Shop (shop_id));
joinable!(PromotionalOffers -> Shop (shop_id));
joinable!(PushHistory -> PushMessage (messageId));
joinable!(PushHistory -> PushStatistics (statId));
joinable!(PushStatistics -> PushMessage (messageId));
joinable!(PushStatistics -> Shop (shopId));
joinable!(PushTokens -> Shop (shopId));
joinable!(ShopAtos -> Shop (shop_id));
joinable!(ShopAtosTransaction -> Shop (shop_id));
joinable!(ShopAtosTransaction -> User (user_id));
joinable!(ShopCmcic -> Shop (shop_id));
joinable!(ShopCmcicTransaction -> Shop (shop_id));
joinable!(ShopCmcicTransaction -> User (user_id));
joinable!(ShopLydia -> Shop (shop_id));
joinable!(ShopLydiaTransaction -> Shop (shop_id));
joinable!(ShopLydiaTransaction -> User (user_id));
joinable!(ShopOgone -> Shop (shop_id));
joinable!(ShopOgoneTransaction -> Shop (shop_id));
joinable!(ShopOgoneTransaction -> User (user_id));
joinable!(ShopParameters -> Shop (shop_id));
joinable!(ShopPaybox -> Shop (shop_id));
joinable!(ShopPayboxSystem -> Shop (shop_id));
joinable!(ShopPayboxSystemTransaction -> Shop (shop_id));
joinable!(ShopPayboxSystemTransaction -> User (user_id));
joinable!(ShopPayboxTransaction -> Shop (shop_id));
joinable!(ShopPayboxTransaction -> User (user_id));
joinable!(Tasting -> Badge (badge_id));
joinable!(User -> DeliveryAddress (deliveryAddress_id));
joinable!(UserShopGroup -> Shop (shop_id));
joinable!(UserShopGroup -> ShopGroup (ShopGroup_id));
joinable!(UserShopGroup -> User (User_id));
joinable!(ZipcodeCity -> Shop (shop));
joinable!(Zipcodes -> Shop (shop));

allow_tables_to_appear_in_same_query!(
    ApiConnection,
    AppliVersion,
    Badge,
    CocaCola,
    DeliveryAddress,
    Flyers,
    Job,
    JobCvExp,
    JobSpontaneous,
    Navigation,
    NewsletterUser,
    Orangina,
    PayboxSecurity,
    posts,
    PromotionalOffers,
    PushHistory,
    PushMessage,
    PushStatistics,
    PushTokens,
    Settings,
    Shop,
    ShopAtos,
    ShopAtosTransaction,
    ShopCmcic,
    ShopCmcicTransaction,
    ShopGroup,
    ShopLydia,
    ShopLydiaTransaction,
    ShopOgone,
    ShopOgoneTransaction,
    ShopParameters,
    ShopPaybox,
    ShopPayboxSystem,
    ShopPayboxSystemTransaction,
    ShopPayboxTransaction,
    Slider,
    SliderChange,
    SplashScreen,
    Tasting,
    TransactionTemp,
    User,
    UserShopGroup,
    ZipcodeCity,
    Zipcodes,
);
