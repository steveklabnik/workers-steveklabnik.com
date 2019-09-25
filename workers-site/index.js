import { getAssetFromKV } from "@cloudflare/kv-asset-handler";

addEventListener("fetch", event => {
  event.respondWith(handleEvent(event));
});

async function handleEvent(event) {

  // redirect some old URLs
  let pathname = new URL(event.request.url).pathname;

  if (pathname === "/security.html") {
    return Response.redirect("https://www.steveklabnik.com/security/", 301);
  } else if (pathname === "/deleuzional.html") {
    return Response.redirect("https://www.steveklabnik.com/deleuzional/", 301);
  }

  try {
    return await getAssetFromKV(event);
  } catch (e) {
    let pathname = new URL(event.request.url).pathname;
    return new Response(`"${pathname}" not found`, {
      status: 404,
      statusText: "not found"
    });
  }
}
