import Document, { Head, Html, Main, NextScript } from "next/document";
import { GA_TRACKING_ID } from "../components/ga/gtag";

export default class MyDocument extends Document {
  render() {
    return (
      <Html>
        <Head>
          {/* <!-- HTML Meta Tags --> */}
          <title>NYUClassesFromXYZ</title>
          <meta
            name="description"
            content="an NYU search that won't take 2 hours"
          />

          {/* <!-- Facebook Meta Tags --> */}
          <meta
            property="og:url"
            content="https://nyuclassesfrom-xyz.vercel.app"
          />
          <meta property="og:type" content="website" />
          <meta property="og:title" content="NYUClassesFromXYZ" />
          <meta
            property="og:description"
            content="an NYU search that won't take 2 hours"
          />
          <meta
            property="og:image"
            content="https://nyuclassesfrom-xyz.vercel.app/social-image.png"
          />

          {/* <!-- Twitter Meta Tags --> */}
          <meta name="twitter:card" content="summary_large_image" />
          <meta
            property="twitter:domain"
            content="nyuclassesfrom-xyz.vercel.app"
          />
          <meta
            property="twitter:url"
            content="https://nyuclassesfrom-xyz.vercel.app"
          />
          <meta name="twitter:title" content="NYUClassesFromXYZ" />
          <meta
            name="twitter:description"
            content="an NYU search that won't take 2 hours"
          />
          <meta
            name="twitter:image"
            content="https://nyuclassesfrom-xyz.vercel.app/social-image.png"
          />

          {/* Global Site Tag (gtag.js) - Google Analytics */}
          <script
            async
            src={`https://www.googletagmanager.com/gtag/js?id=${GA_TRACKING_ID}`}
          />
          <script
            dangerouslySetInnerHTML={{
              __html: `
              window.dataLayer = window.dataLayer || [];
              function gtag(){dataLayer.push(arguments);}
              gtag('js', new Date());
              gtag('config', '${GA_TRACKING_ID}', {
                page_path: window.location.pathname,
              });
          `,
            }}
          />
        </Head>
        <body className="dark:bg-darkPurple">
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}
