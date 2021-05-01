pub static IMAGE_VIEW_HEAD: &str = r#"<head><title>Image</title></head>
<style>
  img {
    border: 3px solid #555;
    padding: 0;
    display: block;
    margin: 0 auto;
    max-height: 100%;
    max-width: 100%;
  }
  .comment {
    border: 1px solid rgba(16, 46, 46, 1);
    background-color: rgba(66, 233, 245, 0.5);
    float: center;
    border-radius: 5px;
    padding-left: 40px;
    padding-right: 30px;
    padding-top: 10px
  }
  .comment h4,
  .darker span {
    display: inline
  }

</style>
        <body>
          <center>
            <h1>{Title}</h1>
            Image Labels: {img_lables}
            <br><br>
            <img src="{img}" alt="Snow">
            <!-- Comments here -->
            <div>
                <div class="container">
                    <div class="row">
                        <h1>Comments</h1>
                    "#;


pub static IMAGE_VIEW_FOOT: &str = r#"</div>
                </div>
            </div>
            <!-- Form Here -->
              <br><br><br>
            <form action="/comment/{oid}" method="post">
                <label for="name">Name:</label>
                <input type="text" id="name" name="name" required><br><br>
                <label for="name">Comment:</label><br/>
                <textarea id="comment" name="comment" rows="4" cols="50" required></textarea><br/>
                <input type="submit" value="Submit">
            </form>
          </center>
        </body>
</html>"#;

pub static IMAGE_INDEX_VIEW: &str = r#"<html>
<style>
* {
   box-sizing: border-box;
}
h1 {
   text-align: center;
}
.outer-grid {
   display: flex;
  justify-content: center;
   flex-wrap: wrap;
   padding: 0 4px;
}
.inner-grid {
   flex: 25%;
   max-width: 25%;
   padding: 0 4px;
}
.inner-grid img {
   margin-top: 8px;
   width: 100%;
   padding: 10px;
}
@media screen and (max-width: 800px) {
   .inner-grid {
      flex: 50%;
      max-width: 50%;
   }
}
@media screen and (max-width: 600px) {
   .inner-grid {
      flex: 100%;
      max-width: 100%;
   }
}
</style>
<body>
<h1>Ahmad's Image Board</h1>
<center>
<hr>
<br>
<h3>Upload</h3>
<form target="/upload" method="post" enctype="multipart/form-data">
                <label for="title">Picture Title:</label>
                <input type="text" id="title" name="title"required><br><br>
                <input type="file" name="file" accept="image/png, image/jpeg" required/>
                <button type="submit">Submit</button>
</form>
<br>
<hr>
<form target="/" method="post">
        <label for="title">Search:</label>
        <input type="text" id="text" name="text" required>
        <button type="submit">Submit</button>
</form>
</center>
<div class="outer-grid">
<div class="inner-grid">
{grid1}
</div>
<div class="inner-grid">
{grid2}
</div>
<div class="inner-grid">
{grid3}
</div>
</div>
</body>
</html>"#;

pub fn get_comment(author: &str,date: &str, text: &str) -> String {
    format!("<div class='comment text-justify float-left'><h4>{}</h4> <span>- {} </span> <br><p>{}</p></div><br/>",author,date,text)
}
pub fn get_img_div(title: &str, id: &str) -> String {
    format!("<a href='/img/{}'><img src='/get_img/{}'/><figcaption><center>{}</center></figcaption></a>",id,id,title)
}