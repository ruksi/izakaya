import {yukataUrl} from "@/utils";
import {MetadataRoute} from "next";

export default function robots(): MetadataRoute.Robots {
    return {
        rules: {
            userAgent: "*",
            allow: "/",
            disallow: ["/dashboard/", "/settings/"],
        },
        sitemap: `${yukataUrl()}/sitemap.xml`,
    };
}