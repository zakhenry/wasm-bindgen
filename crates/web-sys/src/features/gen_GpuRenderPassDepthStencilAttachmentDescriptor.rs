#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderPassDepthStencilAttachmentDescriptor ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassDepthStencilAttachmentDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachmentDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPassDepthStencilAttachmentDescriptor;
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassDepthStencilAttachmentDescriptor {
    #[cfg(all(feature = "GpuStoreOp", feature = "GpuTextureView",))]
    #[doc = "Construct a new `GpuRenderPassDepthStencilAttachmentDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachmentDescriptor`, `GpuStoreOp`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        attachment: &GpuTextureView,
        depth_load_value: &::wasm_bindgen::JsValue,
        depth_store_op: GpuStoreOp,
        stencil_load_value: &::wasm_bindgen::JsValue,
        stencil_store_op: GpuStoreOp,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.attachment(attachment);
        ret.depth_load_value(depth_load_value);
        ret.depth_store_op(depth_store_op);
        ret.stencil_load_value(stencil_load_value);
        ret.stencil_store_op(stencil_store_op);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    #[doc = "Change the `attachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachmentDescriptor`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn attachment(&mut self, val: &GpuTextureView) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attachment"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthLoadValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachmentDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_load_value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("depthLoadValue"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    #[doc = "Change the `depthStoreOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachmentDescriptor`, `GpuStoreOp`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_store_op(&mut self, val: GpuStoreOp) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("depthStoreOp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stencilLoadValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachmentDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_load_value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stencilLoadValue"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    #[doc = "Change the `stencilStoreOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachmentDescriptor`, `GpuStoreOp`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_store_op(&mut self, val: GpuStoreOp) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stencilStoreOp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
