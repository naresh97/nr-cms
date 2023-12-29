pub const PAGE_LOGIC: &'static str = r##"
<script type="text/javascript">
addEventListener("load", (event)=>{
    const current_page = new URLSearchParams(window.location.search).get("page") ?? "home";
    const current_page_id = "page-" + current_page;
    document.getElementById(current_page_id).style.visibility = "visible";
});
</script>
"##;
