impl serde::Serialize for AdvancedExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.optimization.is_empty() {
            len += 1;
        }
        if self.enhancement.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("substrait.extensions.AdvancedExtension", len)?;
        if !self.optimization.is_empty() {
            struct_ser.serialize_field("optimization", &self.optimization)?;
        }
        if let Some(v) = self.enhancement.as_ref() {
            struct_ser.serialize_field("enhancement", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdvancedExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "optimization",
            "enhancement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Optimization,
            Enhancement,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "optimization" => Ok(GeneratedField::Optimization),
                            "enhancement" => Ok(GeneratedField::Enhancement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdvancedExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct substrait.extensions.AdvancedExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AdvancedExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut optimization__ = None;
                let mut enhancement__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Optimization => {
                            if optimization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optimization"));
                            }
                            optimization__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Enhancement => {
                            if enhancement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enhancement"));
                            }
                            enhancement__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AdvancedExtension {
                    optimization: optimization__.unwrap_or_default(),
                    enhancement: enhancement__,
                })
            }
        }
        deserializer.deserialize_struct("substrait.extensions.AdvancedExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SimpleExtensionDeclaration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.mapping_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("substrait.extensions.SimpleExtensionDeclaration", len)?;
        if let Some(v) = self.mapping_type.as_ref() {
            match v {
                simple_extension_declaration::MappingType::ExtensionType(v) => {
                    struct_ser.serialize_field("extensionType", v)?;
                }
                simple_extension_declaration::MappingType::ExtensionTypeVariation(v) => {
                    struct_ser.serialize_field("extensionTypeVariation", v)?;
                }
                simple_extension_declaration::MappingType::ExtensionFunction(v) => {
                    struct_ser.serialize_field("extensionFunction", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SimpleExtensionDeclaration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extension_type",
            "extensionType",
            "extension_type_variation",
            "extensionTypeVariation",
            "extension_function",
            "extensionFunction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExtensionType,
            ExtensionTypeVariation,
            ExtensionFunction,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "extensionType" | "extension_type" => Ok(GeneratedField::ExtensionType),
                            "extensionTypeVariation" | "extension_type_variation" => Ok(GeneratedField::ExtensionTypeVariation),
                            "extensionFunction" | "extension_function" => Ok(GeneratedField::ExtensionFunction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SimpleExtensionDeclaration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct substrait.extensions.SimpleExtensionDeclaration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SimpleExtensionDeclaration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mapping_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExtensionType => {
                            if mapping_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionType"));
                            }
                            mapping_type__ = map_.next_value::<::std::option::Option<_>>()?.map(simple_extension_declaration::MappingType::ExtensionType)
;
                        }
                        GeneratedField::ExtensionTypeVariation => {
                            if mapping_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionTypeVariation"));
                            }
                            mapping_type__ = map_.next_value::<::std::option::Option<_>>()?.map(simple_extension_declaration::MappingType::ExtensionTypeVariation)
;
                        }
                        GeneratedField::ExtensionFunction => {
                            if mapping_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionFunction"));
                            }
                            mapping_type__ = map_.next_value::<::std::option::Option<_>>()?.map(simple_extension_declaration::MappingType::ExtensionFunction)
;
                        }
                    }
                }
                Ok(SimpleExtensionDeclaration {
                    mapping_type: mapping_type__,
                })
            }
        }
        deserializer.deserialize_struct("substrait.extensions.SimpleExtensionDeclaration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for simple_extension_declaration::ExtensionFunction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.extension_uri_reference != 0 {
            len += 1;
        }
        if self.extension_urn_reference != 0 {
            len += 1;
        }
        if self.function_anchor != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("substrait.extensions.SimpleExtensionDeclaration.ExtensionFunction", len)?;
        if self.extension_uri_reference != 0 {
            struct_ser.serialize_field("extensionUriReference", &self.extension_uri_reference)?;
        }
        if self.extension_urn_reference != 0 {
            struct_ser.serialize_field("extensionUrnReference", &self.extension_urn_reference)?;
        }
        if self.function_anchor != 0 {
            struct_ser.serialize_field("functionAnchor", &self.function_anchor)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for simple_extension_declaration::ExtensionFunction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extension_uri_reference",
            "extensionUriReference",
            "extension_urn_reference",
            "extensionUrnReference",
            "function_anchor",
            "functionAnchor",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExtensionUriReference,
            ExtensionUrnReference,
            FunctionAnchor,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "extensionUriReference" | "extension_uri_reference" => Ok(GeneratedField::ExtensionUriReference),
                            "extensionUrnReference" | "extension_urn_reference" => Ok(GeneratedField::ExtensionUrnReference),
                            "functionAnchor" | "function_anchor" => Ok(GeneratedField::FunctionAnchor),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = simple_extension_declaration::ExtensionFunction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct substrait.extensions.SimpleExtensionDeclaration.ExtensionFunction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<simple_extension_declaration::ExtensionFunction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extension_uri_reference__ = None;
                let mut extension_urn_reference__ = None;
                let mut function_anchor__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExtensionUriReference => {
                            if extension_uri_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionUriReference"));
                            }
                            extension_uri_reference__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExtensionUrnReference => {
                            if extension_urn_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionUrnReference"));
                            }
                            extension_urn_reference__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FunctionAnchor => {
                            if function_anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionAnchor"));
                            }
                            function_anchor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(simple_extension_declaration::ExtensionFunction {
                    extension_uri_reference: extension_uri_reference__.unwrap_or_default(),
                    extension_urn_reference: extension_urn_reference__.unwrap_or_default(),
                    function_anchor: function_anchor__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("substrait.extensions.SimpleExtensionDeclaration.ExtensionFunction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for simple_extension_declaration::ExtensionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.extension_uri_reference != 0 {
            len += 1;
        }
        if self.extension_urn_reference != 0 {
            len += 1;
        }
        if self.type_anchor != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("substrait.extensions.SimpleExtensionDeclaration.ExtensionType", len)?;
        if self.extension_uri_reference != 0 {
            struct_ser.serialize_field("extensionUriReference", &self.extension_uri_reference)?;
        }
        if self.extension_urn_reference != 0 {
            struct_ser.serialize_field("extensionUrnReference", &self.extension_urn_reference)?;
        }
        if self.type_anchor != 0 {
            struct_ser.serialize_field("typeAnchor", &self.type_anchor)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for simple_extension_declaration::ExtensionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extension_uri_reference",
            "extensionUriReference",
            "extension_urn_reference",
            "extensionUrnReference",
            "type_anchor",
            "typeAnchor",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExtensionUriReference,
            ExtensionUrnReference,
            TypeAnchor,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "extensionUriReference" | "extension_uri_reference" => Ok(GeneratedField::ExtensionUriReference),
                            "extensionUrnReference" | "extension_urn_reference" => Ok(GeneratedField::ExtensionUrnReference),
                            "typeAnchor" | "type_anchor" => Ok(GeneratedField::TypeAnchor),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = simple_extension_declaration::ExtensionType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct substrait.extensions.SimpleExtensionDeclaration.ExtensionType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<simple_extension_declaration::ExtensionType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extension_uri_reference__ = None;
                let mut extension_urn_reference__ = None;
                let mut type_anchor__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExtensionUriReference => {
                            if extension_uri_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionUriReference"));
                            }
                            extension_uri_reference__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExtensionUrnReference => {
                            if extension_urn_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionUrnReference"));
                            }
                            extension_urn_reference__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TypeAnchor => {
                            if type_anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeAnchor"));
                            }
                            type_anchor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(simple_extension_declaration::ExtensionType {
                    extension_uri_reference: extension_uri_reference__.unwrap_or_default(),
                    extension_urn_reference: extension_urn_reference__.unwrap_or_default(),
                    type_anchor: type_anchor__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("substrait.extensions.SimpleExtensionDeclaration.ExtensionType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for simple_extension_declaration::ExtensionTypeVariation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.extension_uri_reference != 0 {
            len += 1;
        }
        if self.extension_urn_reference != 0 {
            len += 1;
        }
        if self.type_variation_anchor != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("substrait.extensions.SimpleExtensionDeclaration.ExtensionTypeVariation", len)?;
        if self.extension_uri_reference != 0 {
            struct_ser.serialize_field("extensionUriReference", &self.extension_uri_reference)?;
        }
        if self.extension_urn_reference != 0 {
            struct_ser.serialize_field("extensionUrnReference", &self.extension_urn_reference)?;
        }
        if self.type_variation_anchor != 0 {
            struct_ser.serialize_field("typeVariationAnchor", &self.type_variation_anchor)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for simple_extension_declaration::ExtensionTypeVariation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extension_uri_reference",
            "extensionUriReference",
            "extension_urn_reference",
            "extensionUrnReference",
            "type_variation_anchor",
            "typeVariationAnchor",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExtensionUriReference,
            ExtensionUrnReference,
            TypeVariationAnchor,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "extensionUriReference" | "extension_uri_reference" => Ok(GeneratedField::ExtensionUriReference),
                            "extensionUrnReference" | "extension_urn_reference" => Ok(GeneratedField::ExtensionUrnReference),
                            "typeVariationAnchor" | "type_variation_anchor" => Ok(GeneratedField::TypeVariationAnchor),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = simple_extension_declaration::ExtensionTypeVariation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct substrait.extensions.SimpleExtensionDeclaration.ExtensionTypeVariation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<simple_extension_declaration::ExtensionTypeVariation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extension_uri_reference__ = None;
                let mut extension_urn_reference__ = None;
                let mut type_variation_anchor__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExtensionUriReference => {
                            if extension_uri_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionUriReference"));
                            }
                            extension_uri_reference__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExtensionUrnReference => {
                            if extension_urn_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionUrnReference"));
                            }
                            extension_urn_reference__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TypeVariationAnchor => {
                            if type_variation_anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeVariationAnchor"));
                            }
                            type_variation_anchor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(simple_extension_declaration::ExtensionTypeVariation {
                    extension_uri_reference: extension_uri_reference__.unwrap_or_default(),
                    extension_urn_reference: extension_urn_reference__.unwrap_or_default(),
                    type_variation_anchor: type_variation_anchor__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("substrait.extensions.SimpleExtensionDeclaration.ExtensionTypeVariation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SimpleExtensionUri {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.extension_uri_anchor != 0 {
            len += 1;
        }
        if !self.uri.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("substrait.extensions.SimpleExtensionURI", len)?;
        if self.extension_uri_anchor != 0 {
            struct_ser.serialize_field("extensionUriAnchor", &self.extension_uri_anchor)?;
        }
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SimpleExtensionUri {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extension_uri_anchor",
            "extensionUriAnchor",
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExtensionUriAnchor,
            Uri,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "extensionUriAnchor" | "extension_uri_anchor" => Ok(GeneratedField::ExtensionUriAnchor),
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SimpleExtensionUri;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct substrait.extensions.SimpleExtensionURI")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SimpleExtensionUri, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extension_uri_anchor__ = None;
                let mut uri__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExtensionUriAnchor => {
                            if extension_uri_anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionUriAnchor"));
                            }
                            extension_uri_anchor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SimpleExtensionUri {
                    extension_uri_anchor: extension_uri_anchor__.unwrap_or_default(),
                    uri: uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("substrait.extensions.SimpleExtensionURI", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SimpleExtensionUrn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.extension_urn_anchor != 0 {
            len += 1;
        }
        if !self.urn.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("substrait.extensions.SimpleExtensionURN", len)?;
        if self.extension_urn_anchor != 0 {
            struct_ser.serialize_field("extensionUrnAnchor", &self.extension_urn_anchor)?;
        }
        if !self.urn.is_empty() {
            struct_ser.serialize_field("urn", &self.urn)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SimpleExtensionUrn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extension_urn_anchor",
            "extensionUrnAnchor",
            "urn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExtensionUrnAnchor,
            Urn,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "extensionUrnAnchor" | "extension_urn_anchor" => Ok(GeneratedField::ExtensionUrnAnchor),
                            "urn" => Ok(GeneratedField::Urn),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SimpleExtensionUrn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct substrait.extensions.SimpleExtensionURN")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SimpleExtensionUrn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extension_urn_anchor__ = None;
                let mut urn__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExtensionUrnAnchor => {
                            if extension_urn_anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionUrnAnchor"));
                            }
                            extension_urn_anchor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Urn => {
                            if urn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urn"));
                            }
                            urn__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SimpleExtensionUrn {
                    extension_urn_anchor: extension_urn_anchor__.unwrap_or_default(),
                    urn: urn__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("substrait.extensions.SimpleExtensionURN", FIELDS, GeneratedVisitor)
    }
}
