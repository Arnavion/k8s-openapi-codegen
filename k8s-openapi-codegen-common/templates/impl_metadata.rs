
impl{type_generics_impl} {crate_root}::Metadata for {type_name}{type_generics_type}{type_generics_where} {{
    type Ty = {metadata_type_name};

    fn metadata(&self) -> Option<&<Self as {0}::Metadata>::Ty> {{
        {metadata_expr}
    }}

    fn metadata_mut(&mut self) -> Option<&mut<Self as {0}::Metadata>::Ty> {{
        {metadata_mut_expr}
    }}

    fn set_metadata(&mut self, metadata: <Self as {0}::Metadata>::Ty) {{
        {set_metadata_expr}
    }}

}}