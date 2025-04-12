# Custom Matchers

You can easily extend FluentTest with your own matchers:

```rust
// Define a custom matcher for your domain
trait UserMatchers<T> {
    fn to_be_admin(self) -> Self;
}

// Implement it for the Assertion type
impl<T: AsRef<User> + Clone> UserMatchers<T> for Assertion<T> {
    fn to_be_admin(self) -> Self {
        let user = self.value.as_ref();
        let success = user.role == Role::Admin;
        
        // Create a sentence for the assertion
        let sentence = AssertionSentence::new("be", "an admin");
        
        // Add the step to the assertion chain and return it
        return self.add_step(sentence, success);
    }
}

// Use it in your tests
#[test]
fn test_user_permissions() {
    // Enable enhanced output for more descriptive messages
    config().enhanced_output(true).apply();
    
    let admin_user = get_admin_user();
    let regular_user = get_regular_user();
    
    expect!(admin_user).to_be_admin();           // Passes
    expect!(regular_user).not().to_be_admin();   // Passes
}
```

## Creating Domain-Specific Matchers

For more complex domains, you can create a whole set of related matchers:

```rust
// Domain-specific matcher traits
trait UserMatchers<T> {
    fn to_be_admin(self) -> Self;
    fn to_have_permission(self, permission: Permission) -> Self;
    fn to_be_in_group(self, group: &str) -> Self;
}

// Implement for assertions
impl<T: AsRef<User> + Clone> UserMatchers<T> for Assertion<T> {
    fn to_be_admin(self) -> Self {
        let user = self.value.as_ref();
        let success = user.role == Role::Admin;
        
        let sentence = AssertionSentence::new("be", "an admin");
        return self.add_step(sentence, success);
    }
    
    fn to_have_permission(self, permission: Permission) -> Self {
        let user = self.value.as_ref();
        let success = user.permissions.contains(&permission);
        
        let sentence = AssertionSentence::new("have", 
            &format!("permission {:?}", permission));
        return self.add_step(sentence, success);
    }
    
    fn to_be_in_group(self, group: &str) -> Self {
        let user = self.value.as_ref();
        let success = user.groups.contains(&group.to_string());
        
        let sentence = AssertionSentence::new("be", 
            &format!("in group {}", group));
        return self.add_step(sentence, success);
    }
}
```

## Best Practices

When creating custom matchers, follow these guidelines:

1. **Be descriptive**: Use clear verb-object combinations in your assertions
2. **Return self**: Always return `self` to support chaining
3. **Use `AssertionSentence`**: Create a sentence that reads naturally
4. **Support negation**: Ensure your matcher works correctly with the `.not()` modifier
5. **Clone when needed**: Use `.clone()` for values that need to be used after the assertion
6. **Keep it focused**: Each matcher should test one specific property
7. **Add documentation**: Document your matchers with examples

## Complete Example

Here's a complete example of a custom matcher for a blog post domain:

```rust
struct Post {
    title: String,
    content: String,
    published: bool,
    tags: Vec<String>,
    author: User,
}

trait PostMatchers<T> {
    fn to_be_published(self) -> Self;
    fn to_have_tag(self, tag: &str) -> Self;
    fn to_have_content_containing(self, text: &str) -> Self;
}

impl<T: AsRef<Post> + Clone> PostMatchers<T> for Assertion<T> {
    fn to_be_published(self) -> Self {
        let post = self.value.as_ref();
        let success = post.published;
        
        let sentence = AssertionSentence::new("be", "published");
        return self.add_step(sentence, success);
    }
    
    fn to_have_tag(self, tag: &str) -> Self {
        let post = self.value.as_ref();
        let success = post.tags.contains(&tag.to_string());
        
        let sentence = AssertionSentence::new("have", &format!("tag '{}'", tag));
        return self.add_step(sentence, success);
    }
    
    fn to_have_content_containing(self, text: &str) -> Self {
        let post = self.value.as_ref();
        let success = post.content.contains(text);
        
        let sentence = AssertionSentence::new("have", 
            &format!("content containing '{}'", text));
        return self.add_step(sentence, success);
    }
}

// Usage in tests
#[test]
fn test_blog_post() {
    let post = get_blog_post();
    
    expect!(post)
        .to_be_published()
        .and().to_have_tag("rust")
        .and().to_have_content_containing("FluentTest");
}
```