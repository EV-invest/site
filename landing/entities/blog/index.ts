// `blog` entity — the domain-facing surface over the generated API client.
//
// The transport (typed SDK functions, request/response types) is generated
// from the backend contract under @/shared/api. This slice re-exports it under
// stable domain names so feature/view code depends on "the blog entity" rather
// than reaching into shared/api directly. Add domain mappers/selectors here as
// the UI grows; keep the generated layer untouched.
export {
  type BlogResponse as Blog,
  type CreateBlogRequest as NewBlog,
  createBlog,
  getBlog,
  listBlogs,
} from "@/shared/api";
