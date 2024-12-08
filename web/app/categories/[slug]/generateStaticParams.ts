// app/categories/[slug]/generateStaticParams.ts
export async function fetchCategories() {
    // 这里应该是你获取类别数据的逻辑
    // 例如，从API获取类别数据
    return [
      { slug: 'category-1' },
      { slug: 'category-2' },
      // 更多类别...
    ];
  }
  
  export async function generateStaticParams() {
    const categories = await fetchCategories();
    return categories.map(category => ({
      slug: category.slug,
    }));
  }