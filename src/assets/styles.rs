pub static SITE_STYLE: &str = r#"
<style type="text/css">
body{
    display: flex;
    flex-direction: column;
    align-items: center;
}
.link_image{
    height: 1em;
    width: auto;
    vertical-align: text-top;
}
.page{
    visibility: collapse;
    display: flex;
    flex-direction: column;
    align-items: center;
}
.blog-post h2{
    margin-bottom: 0.25em;
}
.blog-post-date{
    font-size: small;
    color: grey;
}
</style>
"#;
