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
            <form target="/comment" method="post" enctype="multipart/form-data">
                <label for="name">Name:</label>
                <input type="text" id="name" name="name"><br><br>
                <label for="name">Comment:</label><br/>
                <textarea id="comment" name="comment" rows="4" cols="50"></textarea><br/>
                <button type="submit">Submit</button>
            </form>
          </center>
        </body>
</html>"#;

pub fn get_comment(author: &str,date: &str, text: &str) -> String {
    format!("<div class='comment text-justify float-left'><h4>{}</h4> <span> {} </span> <br><p>{}</p></div><br/>",author,date,text)
}