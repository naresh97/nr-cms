pub const PAGE_LOGIC: &'static str = r##"
<script type="text/javascript">
addEventListener("load", (event)=>{
    const current_page = new URLSearchParams(window.location.search).get("page") ?? "home";
    const current_page_id = "page-" + current_page;
    document.getElementById(current_page_id).style.visibility = "visible";

    let date_elems = document.getElementsByClassName("blog-post-date");
    for(let date_elem of date_elems){
        const utc = Number(date_elem.innerHTML);
        const utcString = new Date(utc).toLocaleString();
        date_elem.innerHTML = utcString;
    }
});
</script>
"##;
