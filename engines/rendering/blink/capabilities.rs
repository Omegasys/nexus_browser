// Nexus Browser Blink Capabilities
// GPL-3.0 License


#[derive(Debug, Clone)]
pub struct BlinkCapabilities {

    pub html: bool,

    pub css: bool,

    pub javascript: bool,

    pub webassembly: bool,

    pub webgl: bool,

    pub webgpu: bool,

    pub web_audio: bool,

    pub webrtc: bool,

    pub service_workers: bool,

    pub indexeddb: bool,

}


impl BlinkCapabilities {


    pub fn standard() -> Self {

        Self {

            html: true,

            css: true,

            javascript: true,

            webassembly: true,

            webgl: true,

            webgpu: true,

            web_audio: true,

            webrtc: true,

            service_workers: true,

            indexeddb: true,

        }

    }


    pub fn disabled() -> Self {

        Self {

            html: false,

            css: false,

            javascript: false,

            webassembly: false,

            webgl: false,

            webgpu: false,

            web_audio: false,

            webrtc: false,

            service_workers: false,

            indexeddb: false,

        }

    }


    pub fn supports(

        &self,

        capability: &str

    ) -> bool {

        match capability {

            "html" =>
                self.html,

            "css" =>
                self.css,

            "javascript" =>
                self.javascript,

            "webassembly" =>
                self.webassembly,

            "webgl" =>
                self.webgl,

            "webgpu" =>
                self.webgpu,

            "web_audio" =>
                self.web_audio,

            "webrtc" =>
                self.webrtc,

            "service_workers" =>
                self.service_workers,

            "indexeddb" =>
                self.indexeddb,

            _ =>
                false,

        }

    }

}
