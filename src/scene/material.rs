use ffi::*;

define_type_and_iterator_indirect! {
    /// Material type (not yet implemented)
    struct Material(&AiMaterial)
    /// Material iterator type.
    struct MaterialIter
}

impl<'a> Material<'a> {
    pub fn num_textures(&self, kind: AiTextureType) -> u32 {
        unsafe { aiGetMaterialTextureCount(self.0, kind) }
    }

    pub fn get_texture(&self, kind: AiTextureType, index: u32, name: &str) {
        let mut aistr: AiString = From::from(name);
        unsafe {
            aiGetMaterialTexture(
                self.0,
                kind,
                index,
                &mut aistr,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut());
        }
    }
}
