import {defineConfig} from 'vitepress'
import {useSidebar} from 'vitepress-openapi'
import spec from '../src/openapi.json' with {type: 'json'}

const sidebar = useSidebar({spec})

export default defineConfig({
    title: "The Elder Scrobz",
    description: "A music tracking application that helps you monitor and analyze your listening habits.",
    themeConfig: {
        // https://vitepress.dev/reference/default-theme-config
        nav: [
            {text: 'Home', link: '/'},
            {text: 'Examples', link: '/markdown-examples'}
        ],

        sidebar: [
            {
                text: 'Examples',
                items: [
                    {text: 'Markdown Examples', link: '/markdown-examples'},
                    {text: 'Runtime API Examples', link: '/api-examples'}
                ]
            },
            ...sidebar.generateSidebarGroups({
                linkPrefix: "/operations/"
            }),
        ],

        socialLinks: [
            {icon: 'github', link: 'https://github.com/vuejs/vitepress'}
        ]
    }
})
