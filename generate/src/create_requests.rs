use futures::future::join_all;

pub async fn create_requests(client: &reqwest::Client, annotation_langs: &Vec<&str>) -> reqwest::Result<Vec<String>> {
    let annotation_requests: Vec<reqwest::Response> = join_all(
	annotation_langs.iter().map(|lang| {
	    client.get(&format!("https://raw.githubusercontent.com/unicode-org/cldr/release-38/common/annotations/{}.xml", lang)).send()
	})).await.into_iter().collect::<Result<_, _>>()?;
    join_all(annotation_requests.into_iter().map(|r| {
	r.text()
    })).await.into_iter().collect::<Result<_, _>>()
}
