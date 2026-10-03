# Engine marks

These files identify the graph engines compared in the technical guide. They are stored
locally so the page never hotlinks a vendor, and every file came from an official vendor
source. All trademarks belong to their owners. Showing a mark does not imply that the
vendor endorses, sponsors, or partners with this showcase.

Rules for anyone changing these files:

- Do not recolour, redraw, crop, trace, rotate, or distort a mark.
- Do not take marks from aggregators, favicons, or search-result thumbnails.
- Always show the product name as visible text next to the mark. Mark images use `alt=""`
  because the name beside them already carries the meaning.
- Keep the clear space the original artwork provides, and display each mark at its own
  aspect ratio (`object-fit: contain`).
- If a mark cannot be verified, or its reuse terms are unsuitable, use the neutral
  monogram fallback (`logo:null`) instead.

Retrieved on 2 October 2026.

## Marks in use

| File | Vendor and product | Source | Original file |
| --- | --- | --- | --- |
| `neo4j.svg` | Neo4j, Inc. — Neo4j | Neo4j website and documentation header logo: <https://dist.neo4j.com/wp-content/uploads/20230926084108/Logo_FullColor_RGB_TransBG.svg> (brand portal: <https://neo4j.com/brand/>) | `Logo_FullColor_RGB_TransBG.svg` (SVG, full colour) |
| `falkordb.png` | FalkorDB Ltd. — FalkorDB | Official FalkorDB repository README, image "FalkorDB Logo Square B": <https://github.com/user-attachments/assets/ec67e5e3-a060-405d-b229-1fe2badcf8ad> (repository: <https://github.com/FalkorDB/FalkorDB>) | PNG, 508 × 508. No vendor SVG was published, so the PNG is used unchanged. |
| `memgraph.svg` | Memgraph Ltd. — Memgraph | Memgraph documentation navigation logo: <https://memgraph.com/docs/memgraph-logo-navigation.svg> | `memgraph-logo-navigation.svg` (SVG) |
| `google-cloud-spanner.svg` | Google LLC — Google Cloud Spanner (Spanner Graph) | Google Cloud core product icons: <https://services.google.com/fh/files/misc/core-products-icons.zip>, path `Unique Icons/Cloud Spanner/SVG/`, listed on <https://cloud.google.com/icons> | `CloudSpanner-512-color.svg` (SVG, colour) |
| `puppygraph.svg` | PuppyGraph — PuppyGraph | PuppyGraph website header logo: <https://cdn.prod.website-files.com/65d609edcc331dd0e4eb5167/65d7517f9f226df9af7bf547_Horizontal_purple.svg> | `Horizontal_purple.svg` (SVG, the vendor's default purple variant) |
| `microsoft-fabric.svg` | Microsoft Corporation — Microsoft Fabric | Microsoft Fabric icons package, linked from <https://learn.microsoft.com/en-us/fabric/fundamentals/icons>: <https://github.com/microsoft/fabric-samples/raw/main/docs-samples/Icons.zip>, path `v6.1.0/package/dist/svg/` (package `@fabric-msft/svg-icons`, MIT licence) | `fabric_48_color.svg` (SVG, colour) |
| `mongodb.svg` | MongoDB, Inc. — MongoDB Atlas | MongoDB brand resources: <https://www.mongodb.com/company/newsroom/brand-resources> (press kit: <https://brandfolder.com/mongodb/press-kit>), file <https://webimages.mongodb.com/_com_assets/cms/kuyjf3vea2hg34taa-horizontal_default_slate_blue.svg> | Horizontal default wordmark, slate blue (SVG) |
| `azure-cosmos-db.svg` | Microsoft Corporation — Azure Cosmos DB | Azure public service icons V24, linked from <https://learn.microsoft.com/en-us/azure/architecture/icons/>: <https://arch-center.azureedge.net/icons/Azure_Public_Service_Icons_V24.zip> | `10121-icon-service-Azure-Cosmos-DB.svg` (SVG) |

## Notes on terms and preparation

- **Google Cloud Spanner.** Google Cloud describes its official icon library as being for
  "your diagrams, technical documentation, and more". The technical guide is technical
  documentation, so the icon is used in that context, always next to the product name.
- **Microsoft Fabric.** The icons package says the icons are intended for Microsoft Fabric
  platform extension development. The code is MIT licensed, but the Fabric name and mark
  remain Microsoft trademarks, so the mark only identifies Microsoft Fabric and always
  appears with its name. The Azure "Service Fabric" icon is a different product and must
  not be used here.
- **Azure Cosmos DB.** The Azure icon FAQ forbids cropping, flipping, rotating, or
  distorting icons. It requires the full service name to appear near the icon, and an icon
  may only represent the product it was designed for. The row is therefore labelled
  "Azure Cosmos DB for Apache Gremlin".
- **Neo4j, FalkorDB, Memgraph, PuppyGraph and MongoDB.** Each mark is the vendor's own
  published artwork, used only to identify that vendor's product in a comparison, with the
  name always visible.
- **Preparation.** Only non-visual changes were made. The Adobe Illustrator generator
  comments were removed from `memgraph.svg` and `google-cloud-spanner.svg`, and trailing
  whitespace was trimmed from `memgraph.svg` and `neo4j.svg`. The Fabric
  icon had `width="48" height="48"` but no `viewBox`, so `viewBox="0 0 48 48"` was added,
  matching its own canvas, so it scales inside a fixed slot. No paths, colours, or
  proportions were changed. No file contains scripts, event handlers, or remote
  references.

## Monogram fallbacks

| Engine | Reason |
| --- | --- |
| Amazon Neptune | The AWS architecture icons are licensed for creating architecture diagrams. A comparison-chart label is not an architecture diagram, so the AWS icon is not used. |
| Kuzu forks | The Kuzu website and documentation domains no longer resolve, and the archived repository does not publish a logo, so no official mark could be verified. Forks are separate projects without a single shared mark. |
| Kuzu 0.11.3 | Same reason as above: no verifiable official mark for the archived project. |
